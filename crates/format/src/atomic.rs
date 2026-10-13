//! Safe saving: a file is written to a temporary file beside it, then renamed over it, so a failed
//! or interrupted write never leaves a damaged file behind (the old one stays as it was).
//! [`write_new_with`] creates a file the same way without ever replacing one.

use std::fs::{File, OpenOptions};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Write `bytes` to `path` atomically (see the module docs).
pub fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    write_atomic_with(path, |f| f.write_all(bytes))
}

/// Write `path` atomically with `fill`, which writes the content into the temporary file. When
/// anything fails (`fill`, flushing, the rename) the temporary file is removed and `path` is left
/// untouched.
pub fn write_atomic_with(path: &Path, fill: impl FnOnce(&mut File) -> io::Result<()>) -> io::Result<()> {
    let target = resolve_link(path)?;
    let tmp = temp_path(&target)?;
    let mut f = OpenOptions::new().write(true).create_new(true).open(&tmp)?;
    let filled = fill(&mut f).and_then(|()| f.sync_all());
    drop(f);
    filled.and_then(|()| keep_permissions(&target, &tmp)).and_then(|()| std::fs::rename(&tmp, &target)).inspect_err(|_| {
        // Best effort: the temporary file is all there is to clean up.
        let _ = std::fs::remove_file(&tmp);
    })
}

/// Create `path` with the content `fill` writes, never replacing a file or a link of that name. The
/// content goes to a temporary file beside it, which is then linked in under `path`
/// (`std::fs::hard_link`, which fails with `AlreadyExists` when the name is taken, also when it was
/// taken meanwhile). The file appears whole or not at all. Where hard links fail for another
/// reason, `path` is created with `create_new` and the content copied in; a failed copy removes
/// it. The temporary file is always removed.
pub fn write_new_with(path: &Path, fill: impl FnOnce(&mut File) -> io::Result<()>) -> io::Result<()> {
    write_new_linking(path, fill, |tmp, path| std::fs::hard_link(tmp, path))
}

/// [`write_new_with`], with `link` linking the temporary file in under `path`.
fn write_new_linking(
    path: &Path,
    fill: impl FnOnce(&mut File) -> io::Result<()>,
    link: impl FnOnce(&Path, &Path) -> io::Result<()>,
) -> io::Result<()> {
    let tmp = temp_path(path)?;
    let mut f = OpenOptions::new().write(true).create_new(true).open(&tmp)?;
    let filled = fill(&mut f).and_then(|()| f.sync_all());
    drop(f);
    let placed = filled.and_then(|()| match link(&tmp, path) {
        Err(e) if e.kind() != io::ErrorKind::AlreadyExists => copy_new(&tmp, path),
        linked => linked,
    });
    // Best effort: the temporary file is all there is to clean up.
    let _ = std::fs::remove_file(&tmp);
    placed
}

/// Create `path` (never replacing anything) with the content of the file `from`; a failed copy
/// removes it again.
fn copy_new(from: &Path, path: &Path) -> io::Result<()> {
    let mut to = OpenOptions::new().write(true).create_new(true).open(path)?;
    let copied = File::open(from).and_then(|mut f| io::copy(&mut f, &mut to)).and_then(|_| to.sync_all());
    drop(to);
    if copied.is_err() {
        // Best effort: the part copied is all there is to clean up.
        let _ = std::fs::remove_file(path);
    }
    copied
}

/// A symbolic link keeps pointing at its file: write the file it names.
fn resolve_link(path: &Path) -> io::Result<PathBuf> {
    match std::fs::symlink_metadata(path) {
        Ok(m) if m.file_type().is_symlink() => std::fs::canonicalize(path),
        _ => Ok(path.to_path_buf()),
    }
}

/// A fresh hidden name beside `target` (the same folder, so the rename stays on one volume).
fn temp_path(target: &Path) -> io::Result<PathBuf> {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let name = target.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, format!("{}: not a file path", target.display())))?;
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    Ok(target.with_file_name(format!(".{}.{}-{n}.tmp", name.to_string_lossy(), std::process::id())))
}

/// The replacement keeps the replaced file's permissions (Unix; Windows files have none to keep
/// beyond read-only, which makes the save fail as writing in place would).
#[cfg(unix)]
fn keep_permissions(target: &Path, tmp: &Path) -> io::Result<()> {
    match std::fs::metadata(target) {
        Ok(m) => std::fs::set_permissions(tmp, m.permissions()),
        // A new file: the default permissions.
        Err(_) => Ok(()),
    }
}

#[cfg(not(unix))]
fn keep_permissions(_: &Path, _: &Path) -> io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("vc-atomic-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    /// The other files in `d` besides `keep`.
    fn leftovers(d: &Path, keep: &str) -> Vec<String> {
        std::fs::read_dir(d).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().to_string()).filter(|n| n != keep).collect()
    }

    #[test]
    fn the_original_file_survives_a_failed_write() {
        let d = dir("fail");
        let path = d.join("doc.vectorcraft");
        std::fs::write(&path, b"original").unwrap();
        let r = write_atomic_with(&path, |f| {
            f.write_all(b"half a fi")?;
            Err(io::Error::other("disk full"))
        });
        assert!(r.is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"original");
        assert!(leftovers(&d, "doc.vectorcraft").is_empty(), "the temporary file is removed");
        write_atomic(&path, b"new").unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"new");
        assert!(leftovers(&d, "doc.vectorcraft").is_empty());
        // A folder that doesn't exist: an error, nothing written.
        assert!(write_atomic(&d.join("missing").join("x.vectorcraft"), b"x").is_err());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn write_new_never_replaces_what_is_there() {
        let d = dir("new");
        let path = d.join("a.otf");
        write_new_with(&path, |f| f.write_all(b"first")).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"first");
        assert!(leftovers(&d, "a.otf").is_empty(), "the temporary file is removed");
        let r = write_new_with(&path, |f| f.write_all(b"second"));
        assert_eq!(r.map_err(|e| e.kind()), Err(io::ErrorKind::AlreadyExists));
        assert_eq!(std::fs::read(&path).unwrap(), b"first");
        assert!(leftovers(&d, "a.otf").is_empty());
        // A failed write leaves nothing.
        let failed = d.join("b.otf");
        assert!(write_new_with(&failed, |f| f.write_all(b"half").and(Err(io::Error::other("disk full")))).is_err());
        assert!(!failed.exists() && leftovers(&d, "a.otf").is_empty());
        let _ = std::fs::remove_dir_all(&d);
    }

    /// Where hard links fail (a file system without them), the content is copied in, and a taken
    /// name stays as it was. A link that fails because the name is taken copies nothing.
    #[test]
    fn write_new_copies_where_hard_links_fail() {
        let d = dir("new-copy");
        let unsupported = |_: &Path, _: &Path| Err(io::Error::from(io::ErrorKind::Unsupported));
        let path = d.join("c.otf");
        write_new_linking(&path, |f| f.write_all(b"copied"), unsupported).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"copied");
        assert!(leftovers(&d, "c.otf").is_empty(), "the temporary file is removed");
        let r = write_new_linking(&path, |f| f.write_all(b"second"), unsupported);
        assert_eq!(r.map_err(|e| e.kind()), Err(io::ErrorKind::AlreadyExists));
        assert_eq!(std::fs::read(&path).unwrap(), b"copied");
        let taken = |_: &Path, _: &Path| Err(io::Error::from(io::ErrorKind::AlreadyExists));
        let other = d.join("d.otf");
        let r = write_new_linking(&other, |f| f.write_all(b"other"), taken);
        assert_eq!(r.map_err(|e| e.kind()), Err(io::ErrorKind::AlreadyExists));
        assert!(!other.exists() && leftovers(&d, "c.otf").is_empty());
        let _ = std::fs::remove_dir_all(&d);
    }

    /// Concurrent saves are whole-document replacements: never a mix of two serialized files.
    #[test]
    fn simultaneous_writes_finish_with_one_complete_document() {
        let d = dir("concurrent");
        let path = d.join("shared.vectorcraft");
        let payloads: Vec<Vec<u8>> = (0..4).map(|i| vec![i; 8192]).collect();
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(payloads.len()));
        let mut writers = vec![];
        for payload in &payloads {
            let payload = payload.clone();
            let path = path.clone();
            let barrier = barrier.clone();
            writers.push(std::thread::spawn(move || {
                barrier.wait();
                write_atomic(&path, &payload)
            }));
        }
        for writer in writers {
            writer.join().unwrap().unwrap();
        }
        let bytes = std::fs::read(&path).unwrap();
        assert!(payloads.contains(&bytes), "concurrent saves must never mix payloads");
        assert!(leftovers(&d, "shared.vectorcraft").is_empty(), "no temporary files remain");
        let _ = std::fs::remove_dir_all(d);
    }

    #[cfg(unix)]
    #[test]
    fn atomic_write_through_symlink_preserves_the_link_and_original_on_error() {
        let d = dir("atomic-symlink");
        let target = d.join("document.vectorcraft");
        let link = d.join("shortcut.vectorcraft");
        std::fs::write(&target, b"original").unwrap();
        std::os::unix::fs::symlink(&target, &link).unwrap();
        let failed = write_atomic_with(&link, |f| {
            f.write_all(b"unfinished")?;
            Err(io::Error::other("simulated write failure"))
        });
        assert!(failed.is_err());
        assert_eq!(std::fs::read(&link).unwrap(), b"original");
        assert!(link.symlink_metadata().unwrap().file_type().is_symlink());
        write_atomic(&link, b"complete").unwrap();
        assert_eq!(std::fs::read(&target).unwrap(), b"complete");
        assert!(link.symlink_metadata().unwrap().file_type().is_symlink());
        assert_eq!(leftovers(&d, "document.vectorcraft"), ["shortcut.vectorcraft"]);
        let _ = std::fs::remove_dir_all(d);
    }

    /// A link of that name, even one to nothing, is never replaced or written through.
    #[cfg(unix)]
    #[test]
    fn write_new_leaves_links_alone() {
        let d = dir("new-links");
        let target = d.join("target.otf");
        std::fs::write(&target, b"target").unwrap();
        std::os::unix::fs::symlink(&target, d.join("link.otf")).unwrap();
        std::os::unix::fs::symlink(d.join("nothing.otf"), d.join("dangling.otf")).unwrap();
        for name in ["link.otf", "dangling.otf"] {
            let r = write_new_with(&d.join(name), |f| f.write_all(b"new"));
            assert_eq!(r.map_err(|e| e.kind()), Err(io::ErrorKind::AlreadyExists), "{name}");
        }
        assert_eq!(std::fs::read(&target).unwrap(), b"target");
        assert!(!d.join("nothing.otf").exists());
        let mut names: Vec<String> = std::fs::read_dir(&d).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().to_string()).collect();
        names.sort();
        assert_eq!(names, ["dangling.otf", "link.otf", "target.otf"], "no temporary file is left");
        let _ = std::fs::remove_dir_all(&d);
    }
}

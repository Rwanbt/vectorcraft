//! Snapping math shared by tools and smart guides.

use kurbo::Point;

/// Snap a value to a grid with the given spacing and origin.
pub fn snap_to_grid(v: f64, spacing: f64, origin: f64) -> f64 {
    if !v.is_finite() || !origin.is_finite() || !spacing.is_finite() || spacing <= 0.0 {
        return v;
    }
    let snapped = ((v - origin) / spacing).round() * spacing + origin;
    if snapped.is_finite() { snapped } else { v }
}

pub fn snap_point_to_grid(p: Point, spacing: f64) -> Point {
    Point::new(snap_to_grid(p.x, spacing, 0.0), snap_to_grid(p.y, spacing, 0.0))
}

/// A candidate alignment line for smart guides.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
}

/// Find the closest target within `tol` to `v`. Returns (target, delta).
pub fn closest(v: f64, targets: impl IntoIterator<Item = f64>, tol: f64) -> Option<(f64, f64)> {
    targets.into_iter().map(|t| (t, t - v)).filter(|(_, d)| d.abs() <= tol).min_by(|a, b| a.1.abs().total_cmp(&b.1.abs()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid() {
        assert_eq!(snap_to_grid(13.0, 10.0, 0.0), 10.0);
        assert_eq!(snap_to_grid(16.0, 10.0, 0.0), 20.0);
        assert_eq!(snap_to_grid(16.0, 10.0, 3.0), 13.0);
        assert_eq!(snap_to_grid(16.0, 0.0, 0.0), 16.0);
    }

    #[test]
    fn invalid_grid_configuration_never_corrupts_finite_coordinates() {
        assert_eq!(snap_to_grid(13.0, f64::INFINITY, 0.0), 13.0);
        assert_eq!(snap_to_grid(13.0, f64::NAN, 0.0), 13.0);
        assert_eq!(snap_to_grid(13.0, 10.0, f64::INFINITY), 13.0);
        assert_eq!(snap_to_grid(f64::MAX, f64::MIN_POSITIVE, -f64::MAX), f64::MAX);
        assert_eq!(snap_point_to_grid(Point::new(13.0, 16.0), 0.0), Point::new(13.0, 16.0));
        // Valid origin and spacing continue to use the same rounding contract.
        assert_eq!(snap_to_grid(-16.0, 10.0, 3.0), -17.0);
    }

    #[test]
    fn closest_target() {
        assert_eq!(closest(10.0, [0.0, 11.0, 9.5], 2.0), Some((9.5, -0.5)));
        assert_eq!(closest(10.0, [0.0, 20.0], 2.0), None);
    }
}

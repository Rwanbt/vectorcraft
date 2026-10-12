//! Walking a [`BezPath`] as strokes see it: its subpaths (open or closed), their segments, unit
//! tangents and corners. Shared by the stroke geometry (`vectorcraft-effects`) and the visual
//! bounds and hit testing of strokes (`vectorcraft-doc`).

use kurbo::{BezPath, ParamCurve, ParamCurveDeriv, PathEl, PathSeg, Vec2};

/// Tangents meeting at more than about 1° (this cosine) make a corner: fitted dashes centre on it,
/// variable-width strokes give it the stroke's join and miter spikes grow from it.
pub const CORNER_COS: f64 = 0.9998;

/// Element ranges of the subpaths of `bp` (each with whether it is closed).
pub fn subpaths(bp: &BezPath) -> Vec<(std::ops::Range<usize>, bool)> {
    let els = bp.elements();
    let mut out = vec![];
    let mut start = 0;
    for (i, el) in els.iter().enumerate() {
        match el {
            PathEl::MoveTo(_) if i > start => {
                out.push((start..i, false));
                start = i;
            }
            PathEl::ClosePath => {
                out.push((start..i + 1, true));
                start = i + 1;
            }
            _ => {}
        }
    }
    if start < els.len() {
        out.push((start..els.len(), false));
    }
    // A lone MoveTo (or a MoveTo straight after a ClosePath) draws nothing.
    out.retain(|(r, _)| r.len() > 1);
    out
}

/// The segments of one subpath (`els` starts with its MoveTo; a ClosePath adds the closing line).
pub fn segments(els: &[PathEl]) -> Vec<PathSeg> {
    kurbo::segments(els.iter().copied()).collect()
}

/// Unit tangent of `s` at `t`, robust at ends where control points coincide with the end point.
pub fn tangent(s: &PathSeg, t: f64) -> Vec2 {
    let d = match s {
        PathSeg::Line(l) => l.p1 - l.p0,
        PathSeg::Quad(q) => {
            let v = q.deriv().eval(t).to_vec2();
            if v.hypot() > 1e-9 { v } else { q.p2 - q.p0 }
        }
        PathSeg::Cubic(c) => {
            let v = c.deriv().eval(t).to_vec2();
            if v.hypot() > 1e-9 {
                v
            } else if t < 0.5 {
                // The first control point that differs from the start gives the direction.
                [c.p2, c.p3].into_iter().map(|p| p - c.p0).find(|v| v.hypot() > 1e-9).unwrap_or_default()
            } else {
                [c.p1, c.p0].into_iter().map(|p| c.p3 - p).find(|v| v.hypot() > 1e-9).unwrap_or_default()
            }
        }
    };
    unit(d)
}

/// `v` normalised, or +x when it has no length.
pub fn unit(v: Vec2) -> Vec2 {
    let h = v.hypot();
    if h > 1e-12 { v / h } else { Vec2::new(1.0, 0.0) }
}

/// Do segments `a` and `b` (`b` following `a`) meet at a corner?
pub fn kink(a: &PathSeg, b: &PathSeg) -> bool {
    tangent(a, 1.0).dot(tangent(b, 0.0)) < CORNER_COS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_drawing_moves_are_not_subpaths() {
        let mut bp = BezPath::new();
        bp.move_to((0.0, 0.0));
        assert!(subpaths(&bp).is_empty());
        bp.move_to((5.0, 5.0));
        bp.line_to((6.0, 5.0));
        assert_eq!(subpaths(&bp), vec![(1..3, false)]);
    }

    #[test]
    fn degenerate_quadratic_and_cubic_tangents_have_defined_fallbacks() {
        let q = PathSeg::Quad(kurbo::QuadBez::new((0.0, 0.0), (0.0, 0.0), (0.0, 10.0)));
        assert_eq!(tangent(&q, 0.0), Vec2::new(0.0, 1.0), "use the chord when the quadratic derivative vanishes");
        let cq = PathSeg::Quad(kurbo::QuadBez::new((1.0, 1.0), (1.0, 1.0), (1.0, 1.0)));
        assert_eq!(tangent(&cq, 0.5), Vec2::new(1.0, 0.0));
        let start = PathSeg::Cubic(kurbo::CubicBez::new((0.0, 0.0), (0.0, 0.0), (0.0, 0.0), (0.0, 10.0)));
        assert_eq!(tangent(&start, 0.0), Vec2::new(0.0, 1.0), "the last differing control point defines the start direction");
        let end = PathSeg::Cubic(kurbo::CubicBez::new((0.0, 0.0), (0.0, 10.0), (0.0, 20.0), (0.0, 20.0)));
        assert_eq!(tangent(&end, 1.0), Vec2::new(0.0, 1.0), "the first differing control point defines the end direction");
        let point = PathSeg::Cubic(kurbo::CubicBez::new((5.0, 5.0), (5.0, 5.0), (5.0, 5.0), (5.0, 5.0)));
        assert_eq!(tangent(&point, 0.0), Vec2::new(1.0, 0.0));
        assert_eq!(tangent(&point, 1.0), Vec2::new(1.0, 0.0));
    }

    #[test]
    fn straight_join_is_not_a_kink_but_right_angle_is() {
        let first = PathSeg::Line(kurbo::Line::new((0.0, 0.0), (10.0, 0.0)));
        let straight = PathSeg::Line(kurbo::Line::new((10.0, 0.0), (20.0, 0.0)));
        let corner = PathSeg::Line(kurbo::Line::new((10.0, 0.0), (10.0, 10.0)));
        assert!(!kink(&first, &straight));
        assert!(kink(&first, &corner));
    }

    #[test]
    fn subpaths_and_tangents() {
        let mut bp = BezPath::new();
        bp.move_to((0.0, 0.0));
        bp.line_to((10.0, 0.0));
        bp.line_to((10.0, 10.0));
        bp.close_path();
        bp.move_to((20.0, 0.0));
        bp.curve_to((20.0, 0.0), (30.0, 0.0), (30.0, 10.0));
        let subs = subpaths(&bp);
        assert_eq!(subs, vec![(0..4, true), (4..6, false)]);
        let segs = segments(&bp.elements()[subs[0].0.clone()]);
        assert_eq!(segs.len(), 3, "the close adds a segment");
        assert!(kink(&segs[0], &segs[1]));
        let c = segments(&bp.elements()[subs[1].0.clone()]);
        // A handle on its anchor: the direction comes from the next control point.
        assert_eq!(tangent(&c[0], 0.0), Vec2::new(1.0, 0.0));
        assert_eq!(tangent(&c[0], 1.0), Vec2::new(0.0, 1.0));
        assert_eq!(unit(Vec2::ZERO), Vec2::new(1.0, 0.0));
    }
}

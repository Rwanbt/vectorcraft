//! Reproducible algebraic contracts for the geometry APIs used by drawing tools.
//! Keep the seed explicit: a failing run is replayable on Linux, Windows, and macOS.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use proptest::prelude::*;
use proptest::test_runner::RngSeed;
use vectorcraft_geom::snap::snap_to_grid;
use vectorcraft_geom::{Affine, Homography, Point, normalize_deg};

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 256,
        rng_seed: RngSeed::Fixed(20261012),
        failure_persistence: None,
        ..ProptestConfig::default()
    })]

    /// A grid is translation-equivariant by an integral number of grid cells.
    #[test]
    fn snapped_coordinates_follow_translated_grid_cells(
        x in -1000.0f64..1000.0,
        spacing in 0.25f64..50.0,
        origin in -100.0f64..100.0,
        shift in -10i32..10,
    ) {
        let actual = snap_to_grid(x, spacing, origin);
        let shifted = snap_to_grid(x + f64::from(shift) * spacing, spacing, origin);
        let expected = actual + f64::from(shift) * spacing;
        let epsilon = 1e-8 * (1.0 + x.abs() + origin.abs());
        prop_assert!((shifted - expected).abs() <= epsilon, "{shifted} vs {expected}, spacing={spacing}, origin={origin}");
        prop_assert!((actual - x).abs() <= spacing * 0.5 + epsilon, "the nearest grid cell must be within half a spacing");
    }

    /// A projective representation of an affine map agrees with the independent affine operator,
    /// and inversion restores the input point with bounded numerical error.
    #[test]
    fn affine_homography_has_exact_forward_and_inverse_contract(
        x in -100.0f64..100.0,
        y in -100.0f64..100.0,
        tx in -100.0f64..100.0,
        ty in -100.0f64..100.0,
        scale in 0.2f64..5.0,
    ) {
        let a = Affine::translate((tx, ty)) * Affine::scale(scale);
        let h = Homography::from_affine(a);
        let input = Point::new(x, y);
        let projected = h.apply(input).unwrap();
        let expected = a * input;
        prop_assert!(projected.distance(expected) < 1e-8, "{projected:?} vs {expected:?}");
        let restored = h.inverse().unwrap().apply(projected).unwrap();
        prop_assert!(restored.distance(input) < 1e-8, "{restored:?} vs {input:?}");
    }

    /// Normalized angles are periodic modulo a full rotation and use (-180, 180].
    #[test]
    fn angle_normalization_is_periodic(
        degrees in -10_000.0f64..10_000.0,
        revolutions in -5i32..5,
    ) {
        let a = normalize_deg(degrees);
        let b = normalize_deg(degrees + 360.0 * f64::from(revolutions));
        prop_assert!(a > -180.0 && a <= 180.0);
        prop_assert!(b > -180.0 && b <= 180.0);
        prop_assert!((a - b).abs() < 1e-8, "different normalized angles: {a} vs {b}");
    }
}

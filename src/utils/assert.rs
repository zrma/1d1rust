use std::str::SplitWhitespace;

#[allow(dead_code)]
pub fn match_multilines_as_f64(i: usize, got: SplitWhitespace, want: SplitWhitespace) {
    for (j, (got, want)) in got.zip(want).enumerate() {
        let got: f64 = got.parse().unwrap();
        let want: f64 = want.parse().unwrap();

        const EPSILON: f64 = 1e-6;

        let abs_diff = (got - want).abs();
        let rel_diff = abs_diff / want.abs().max(1e-9);

        assert!(
            abs_diff < EPSILON || rel_diff < EPSILON,
            "case {}-{}: absolute error: {}, relative error: {}",
            i,
            j,
            abs_diff,
            rel_diff
        );
    }
}

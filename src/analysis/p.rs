#[inline]
pub fn p(pl: f64, pl_new: f64) -> f64 {
    f64::min(1f64, (pl_new - pl).exp())
}

#[inline]
pub fn p(pl: f32, pl_new: f32) -> f32 {
    f32::min(1.0, (pl_new - pl).exp2())
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[link(name = "Accelerate", kind = "framework")]
unsafe extern "C" {
    fn vDSP_dotpr(a: *const f32, ia: isize, b: *const f32, ib: isize, result: *mut f32, n: usize);
}

#[inline(always)]
pub fn pl_pre_log(text: &[f32; 676], ref_log: &[f32; 676]) -> f32 {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    unsafe {
        let mut out = 0.0f32;
        vDSP_dotpr(text.as_ptr(), 1, ref_log.as_ptr(), 1, &mut out, 676);
        return out;
    }
    #[allow(unreachable_code)]
    pl_pre_log_portable(text, ref_log)
}

pub fn pl_pre_log_portable(text: &[f32; 676], ref_log: &[f32; 676]) -> f32 {
    let mut s0 = 0.0f32;
    let mut s1 = 0.0f32;
    let mut s2 = 0.0f32;
    let mut s3 = 0.0f32;

    for i in (0..676).step_by(4) {
        s0 = text[i].mul_add(ref_log[i], s0);
        s1 = text[i + 1].mul_add(ref_log[i + 1], s1);
        s2 = text[i + 2].mul_add(ref_log[i + 2], s2);
        s3 = text[i + 3].mul_add(ref_log[i + 3], s3);
    }

    s0 + s1 + s2 + s3
}

use core::arch::x86_64::*;

#[macro_export]
macro_rules! vsrl_vi2_vi2_i {
  ($x: expr, $c: expr) => {
    unsafe { _mm_srli_epi32($x, $c) }  
  }
}

#[macro_export]
macro_rules! vsra_vi2_vi2_i {
  ($x: expr, $c: expr) => {
    unsafe { _mm_srai_epi32(x, c) }
  }
}

#[inline]
pub fn vcast_vf_f(f: f32) -> __m128 { unsafe { _mm_set1_ps(f) } }
#[inline]
pub fn vreinterpret_vm_vf(vf: __m128) -> __m128i {  unsafe { _mm_castps_si128(vf) } }
#[inline]
pub fn vreinterpret_vf_vm(vm: __m128i) -> __m128 {  unsafe { _mm_castsi128_ps(vm) } }
#[inline]
pub fn vmul_vf_vf_vf(x: __m128, y: __m128) -> __m128 { unsafe { _mm_mul_ps(x, y) } }
#[inline]
pub fn vrint_vi2_vf(vf: __m128) -> __m128i { unsafe { _mm_cvtps_epi32(vf) } }
#[inline]
pub fn vcast_vf_vi2(vi: __m128i) -> __m128 { unsafe { _mm_cvtepi32_ps(vi) } }
#[inline]
pub fn vmla_vf_vf_vf_vf(x: __m128, y: __m128, z: __m128) -> __m128 { unsafe { _mm_fmadd_ps(x, y, z) } }
#[inline]
pub fn vlt_vo_vf_vf(x: __m128, y: __m128) -> __m128i { unsafe { { return vreinterpret_vm_vf(_mm_cmp_ps(x, y, _CMP_LT_OQ)) } }}
#[inline]
pub fn vtestallones_i_vo32(g: __m128i) -> bool { unsafe { _mm_movemask_epi8(g) == 0xFFFF } }
#[inline]
pub fn vabs_vf_vf(f: __m128) -> __m128 { vreinterpret_vf_vm(vandnot_vm_vm_vm(vreinterpret_vm_vf(vcast_vf_f(-0.0)), vreinterpret_vm_vf(f))) }
#[inline]
pub fn vandnot_vm_vm_vm(x: __m128i, y: __m128i) -> __m128i { unsafe { vreinterpret_vm_vd(_mm_andnot_pd(vreinterpret_vd_vm(x), vreinterpret_vd_vm(y))) } }
#[inline]
pub fn vreinterpret_vd_vm(vm: __m128i) -> __m128d { unsafe { _mm_castsi128_pd(vm) } }
#[inline]
pub fn vsel_vf_vo_vf_vf(o: __m128i, x: __m128, y: __m128) -> __m128 { unsafe { _mm_blendv_ps(y, x, _mm_castsi128_ps(o)) } }
#[inline]
pub fn vreinterpret_vm_vd(vd: __m128d) -> __m128i { unsafe { _mm_castpd_si128(vd)  } } 
#[inline]
pub fn vilogb2k_vi2_vf(d: __m128) -> __m128i {
  let q = vreinterpret_vi2_vf(d);
  let q = vsrl_vi2_vi2_i!(q, 23);
  let q = vand_vi2_vi2_vi2(q, vcast_vi2_i(0xff));
  let q = vsub_vi2_vi2_vi2(q, vcast_vi2_i(0x7f));
  return q;
}
#[inline]
pub fn vreinterpret_vi2_vf(vf: __m128) -> __m128i { vreinterpret_vm_vf(vf) }
#[inline]
pub fn vand_vi2_vi2_vi2(x: __m128i, y: __m128i) -> __m128i { unsafe { _mm_and_si128(x, y) } }
#[inline]
pub fn vsub_vi2_vi2_vi2(x: __m128i, y: __m128i) -> __m128i { unsafe { _mm_sub_epi32(x, y) } }
#[inline]
pub fn vcast_vi2_i(i: i32) -> __m128i { unsafe { _mm_set1_epi32(i) } }
#[inline]
pub fn vandnot_vi2_vi2_vi2(x: __m128i, y: __m128i) -> __m128i { unsafe { _mm_andnot_si128(x, y) } }
#[inline]
pub fn vgt_vo_vi2_vi2(x: __m128i, y: __m128i) -> __m128i { unsafe { _mm_cmpgt_epi32(x, y) } }
#[inline]
pub fn vand_vi2_vo_vi2(x: __m128i, y: __m128i) -> __m128i { vand_vi2_vi2_vi2(x, y)  }

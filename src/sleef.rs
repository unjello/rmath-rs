use core::arch::x86_64::*;

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

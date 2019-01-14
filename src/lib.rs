use core::arch::x86_64::{__m128,_mm_set_ps1,_mm_cvtss_f32};

pub struct Vector4 {
  data: __m128
}

impl Vector4 {
  #[inline]
  pub fn new(v: f32) -> Self {
    unsafe {
      let data = _mm_set_ps1(v);  
      Self {
        data
      }
    }
  }

  #[inline]
  pub fn x(&self) -> f32 {
    unsafe {
      _mm_cvtss_f32(self.data)
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn it_works() {
        let v = Vector4::new(0.1);
        assert_approx_eq!(v.x(), 0.1);
    }
}

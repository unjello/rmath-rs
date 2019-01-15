use core::arch::x86_64::{__m128,_mm_set_ps1, _mm_cvtss_f32, _mm_add_ps};
use core::ops::Add;

#[derive(Debug)]
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

impl Add for Vector4 {
  type Output = Vector4;

  fn add(self, other: Vector4) -> Vector4 { 
    unsafe {
      Self {
        data : _mm_add_ps(self.data, other.data)
      }
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn can_be_constructed() {
        let v = Vector4::new(0.1);
        assert_approx_eq!(v.x(), 0.1);
    }

    #[test]
    fn adding_vectors() {
        let v1 = Vector4::new(0.1);
        let v2 = Vector4::new(1.8);
        let v3 = v1 + v2;
        assert_approx_eq!(v3.x(), 1.9);
    }
}

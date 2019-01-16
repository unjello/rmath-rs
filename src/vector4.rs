use core::arch::x86_64::{__m128, _mm_set_ps, _mm_set_ps1, _mm_cvtss_f32, _mm_add_ps, _mm_shuffle_ps};
use core::ops::Add;


#[derive(Debug)]
pub struct Vector4 {
  data: __m128
}

impl Vector4 {
  #[inline]
  pub fn from1(v: f32) -> Self {
    unsafe {
      Self {
        data: _mm_set_ps1(v)
      }
    }
  }

  #[inline]
  pub fn from4(x: f32, y: f32, z: f32, w: f32) -> Self {
    unsafe {
      Self {
        data: _mm_set_ps(w, z, y, x)
      }
    }
  }

  #[inline]
  pub fn x(&self) -> f32 {
    unsafe {
      _mm_cvtss_f32(self.data)
    }
  }

  #[inline]
  pub fn y(&self) -> f32 {
    unsafe {
      _mm_cvtss_f32(_mm_shuffle_ps(self.data, self.data, 0b01010101))
    }
  }

  #[inline]
  pub fn z(&self) -> f32 {
    unsafe {
      _mm_cvtss_f32(_mm_shuffle_ps(self.data, self.data, 0b10101010))
    }
  }

  #[inline]
  pub fn w(&self) -> f32 {
    unsafe {
      _mm_cvtss_f32(_mm_shuffle_ps(self.data, self.data, 0b11111111))
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
    fn can_be_constructed_from1() {
        let v = Vector4::from1(0.1);
        assert_approx_eq!(v.x(), 0.1);
        assert_approx_eq!(v.y(), 0.1);
        assert_approx_eq!(v.z(), 0.1);
        assert_approx_eq!(v.w(), 0.1);
    }

    #[test]
    fn can_be_constructed_from4() {
        let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
        assert_approx_eq!(v.x(), 0.2);
        assert_approx_eq!(v.y(), 1.1);
        assert_approx_eq!(v.z(), -2.9);
        assert_approx_eq!(v.w(), 99.9);
    }

    #[test]
    fn adding_vectors() {
        let v1 = Vector4::from1(0.1);
        let v2 = Vector4::from1(1.8);
        let v3 = v1 + v2;
        assert_approx_eq!(v3.x(), 1.9);
        assert_approx_eq!(v3.y(), 1.9);
        assert_approx_eq!(v3.z(), 1.9);
        assert_approx_eq!(v3.w(), 1.9);
    }
}

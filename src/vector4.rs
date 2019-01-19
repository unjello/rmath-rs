use core::arch::x86_64::{
    __m128, _mm_add_ps, _mm_and_ps, _mm_andnot_ps, _mm_cmplt_ps, _mm_cvtss_f32, _mm_div_ps,
    _mm_floor_ps, _mm_hadd_ps, _mm_mul_ps, _mm_set_ps, _mm_set_ps1, _mm_setzero_ps, _mm_shuffle_ps,
    _mm_sub_ps, _mm_xor_ps,
};
use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug)]
pub struct Vector4 {
    data: __m128,
}

impl Vector4 {
    /// Creates Vector4 instance from one float value.
    /// `x`, `y`, `z` and `w` will be set to the same value.
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from1(0.1);
    /// assert_approx_eq!(v.x(), 0.1);
    /// assert_approx_eq!(v.y(), 0.1);
    /// assert_approx_eq!(v.z(), 0.1);
    /// assert_approx_eq!(v.w(), 0.1);
    /// ```
    #[inline]
    pub fn from1(v: f32) -> Self {
        unsafe {
            Self {
                data: _mm_set_ps1(v),
            }
        }
    }

    /// Creates Vector4 instance from four float values.
    /// `x`, `y`, `z` and `w` will be set to the value
    /// of appropriate parameter.
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// assert_approx_eq!(v.x(), 0.2);
    /// assert_approx_eq!(v.y(), 1.1);
    /// assert_approx_eq!(v.z(), -2.9);
    /// assert_approx_eq!(v.w(), 99.9);
    /// ```  
    #[inline]
    pub fn from4(x: f32, y: f32, z: f32, w: f32) -> Self {
        unsafe {
            Self {
                data: _mm_set_ps(w, z, y, x),
            }
        }
    }

    /// Retrieves `x` component of `Vector4`.
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// assert_approx_eq!(v.x(), 0.2);
    /// ```  
    #[inline]
    pub fn x(&self) -> f32 {
        unsafe { _mm_cvtss_f32(self.data) }
    }

    /// Retrieves `y` component of `Vector4`.
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// assert_approx_eq!(v.y(), 1.1);
    /// ```  
    #[inline]
    pub fn y(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.data, self.data, 0b01010101)) }
    }

    /// Retrieves `z` component of `Vector4`.
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// assert_approx_eq!(v.z(), -2.9);
    /// ```  
    #[inline]
    pub fn z(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.data, self.data, 0b10101010)) }
    }

    /// Retrieves `w` component of `Vector4`.
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// assert_approx_eq!(v.w(), 99.9);
    /// ```  
    #[inline]
    pub fn w(&self) -> f32 {
        unsafe { _mm_cvtss_f32(_mm_shuffle_ps(self.data, self.data, 0b11111111)) }
    }

    /// Finds the nearest integer less than or equal to the parameter
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// let v2 = v.floor();
    /// assert_approx_eq!(v2.x(), 0.0);
    /// assert_approx_eq!(v2.y(), 1.0);
    /// assert_approx_eq!(v2.z(), -3.0);
    /// assert_approx_eq!(v2.w(), 99.0);
    /// ```
    /// See: https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/floor.xhtml
    #[inline]
    pub fn floor(&self) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_floor_ps(self.data),
            }
        }
    }

    /// Finds absolute value of parameter
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// let v2 = v.abs();
    /// assert_approx_eq!(v2.x(), 0.2);
    /// assert_approx_eq!(v2.y(), 1.1);
    /// assert_approx_eq!(v2.z(), 2.9);
    /// assert_approx_eq!(v2.w(), 99.9);
    /// ```
    #[inline]
    pub fn abs(&self) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_andnot_ps(_mm_set_ps1(-0.0), self.data),
            }
        }
    }

    /// Computes the fractional part of the argument
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// let v2 = v.fract();
    /// assert_approx_eq!(v2.x(), 0.2);
    /// assert_approx_eq!(v2.y(), 0.1);
    /// assert_approx_eq!(v2.z(), 0.1);
    /// assert_approx_eq!(v2.w(), 0.9, 1.0e-5); // this one becomes 0.9000015 with regular 1.0e-6 precision :/
    /// ```
    ///
    /// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/fract.xhtml
    #[inline]
    pub fn fract(&self) -> Vector4 {
        let f = self.floor();
        self - &f
    }

    /// Computes value of one parameter modulo another.
    /// Consistent with GLSL implementation
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// let v2 = Vector4::from1(-1.8);
    /// let v3 = v.modulo(&v2);
    /// assert_approx_eq!(v3.x(), -1.6);
    /// assert_approx_eq!(v3.y(), -0.7);
    /// assert_approx_eq!(v3.z(), -1.1);
    /// assert_approx_eq!(v3.w(), -0.9, 1.0e-5);
    /// ```
    ///
    /// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/mod.xhtml
    #[inline]
    pub fn modulo(&self, other: &Vector4) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_sub_ps(
                    self.data,
                    _mm_mul_ps(other.data, _mm_floor_ps(_mm_div_ps(self.data, other.data))),
                ),
            }
        }
    }

    /// Computes value of one parameter modulo another.
    /// Consistent with [Euclidean](https://en.wikipedia.org/wiki/Modulo_operation) division algorithm.
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// let v2 = Vector4::from1(-1.8);
    /// let v3 = v.modulo_euclidean(&v2);
    /// assert_approx_eq!(v3.x(), 0.2);
    /// assert_approx_eq!(v3.y(), 1.1);
    /// assert_approx_eq!(v3.z(), 0.7);
    /// assert_approx_eq!(v3.w(), 0.9, 1.0e-5);
    /// ```
    #[inline]
    pub fn modulo_euclidean(&self, other: &Vector4) -> Vector4 {
        unsafe {
            let ret = self.modulo(&other);
            let mask = _mm_cmplt_ps(ret.data, _mm_setzero_ps());
            Vector4 {
                data: _mm_add_ps(
                    ret.data,
                    _mm_and_ps(mask, _mm_andnot_ps(_mm_set_ps1(-0.0), other.data)),
                ),
            }
        }
    }

    /// Computes value of distance squared between two vectors
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// let v2 = Vector4::from4(0.9, 1.8, 2.9, -14.4);
    /// let d = v1.distance_sq(&v2);
    /// assert_approx_eq!(d, 13099.11);
    /// ```
    #[inline]
    pub fn distance_sq(&self, other: &Vector4) -> f32 {
        unsafe {
            let a_minus_b = _mm_sub_ps(self.data, other.data);
            let a_minus_b_sq = _mm_mul_ps(a_minus_b, a_minus_b);
            let h_add = _mm_hadd_ps(a_minus_b_sq, a_minus_b_sq);
            _mm_cvtss_f32(_mm_hadd_ps(h_add, h_add))
        }
    }

    /// Computes value of distance between two vectors
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// let v2 = Vector4::from4(0.9, 1.8, 2.9, -14.4);
    /// let d = v1.distance(&v2);
    /// assert_approx_eq!(d, 114.45134, 1.0e-5);
    /// ```
    #[inline]
    pub fn distance(&self, other: &Vector4) -> f32 {
        self.distance_sq(other).sqrt()
    }
}

impl Add<&Vector4> for &Vector4 {
    type Output = Vector4;

    /// Implements Add trait for Vector4.
    /// Operator +(Vector4, Vector4).
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from1(0.1);
    /// let v2 = Vector4::from1(1.8);
    /// let v3 = &v1 + &v2;
    /// assert_approx_eq!(v3.x(), 1.9);
    /// assert_approx_eq!(v3.y(), 1.9);
    /// assert_approx_eq!(v3.z(), 1.9);
    /// assert_approx_eq!(v3.w(), 1.9);
    /// ```
    #[inline]
    fn add(self, other: &Vector4) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_add_ps(self.data, other.data),
            }
        }
    }
}

impl Add<f32> for &Vector4 {
    type Output = Vector4;

    /// Implements Add trait for Vector4.
    /// Operator +(Vector4, f32).
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from1(0.1);
    /// let v2 = &v1 + 1.8;
    /// assert_approx_eq!(v2.x(), 1.9);
    /// # assert_approx_eq!(v2.y(), 1.9);
    /// # assert_approx_eq!(v2.z(), 1.9);
    /// # assert_approx_eq!(v2.w(), 1.9);
    /// ```  
    #[inline]
    fn add(self, other: f32) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_add_ps(self.data, _mm_set_ps1(other)),
            }
        }
    }
}

impl Sub for &Vector4 {
    type Output = Vector4;

    /// Implements Sub trait for Vector4.
    /// Operator -(Vector4, Vector4).
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from1(0.1);
    /// let v2 = Vector4::from1(1.8);
    /// let v3 = &v1 - &v2;
    /// assert_approx_eq!(v3.x(), -1.7);
    /// # assert_approx_eq!(v3.y(), -1.7);
    /// # assert_approx_eq!(v3.z(), -1.7);
    /// # assert_approx_eq!(v3.w(), -1.7);
    /// ```  
    #[inline]
    fn sub(self, other: &Vector4) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_sub_ps(self.data, other.data),
            }
        }
    }
}

impl Sub<f32> for &Vector4 {
    type Output = Vector4;

    /// Implements Sub trait for Vector4.
    /// Operator -(Vector4, f32).
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from1(0.1);
    /// let v2 = &v1 - 1.8;
    /// assert_approx_eq!(v2.x(), -1.7);
    /// # assert_approx_eq!(v2.y(), -1.7);
    /// # assert_approx_eq!(v2.z(), -1.7);
    /// # assert_approx_eq!(v2.w(), -1.7);
    /// ```  
    #[inline]
    fn sub(self, other: f32) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_sub_ps(self.data, _mm_set_ps1(other)),
            }
        }
    }
}

impl Mul for &Vector4 {
    type Output = Vector4;

    /// Implements Mul trait for Vector4.
    /// Operator *(Vector4, Vector4).
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from1(0.1);
    /// let v2 = Vector4::from1(1.8);
    /// let v3 = &v1 * &v2;
    /// assert_approx_eq!(v3.x(), 0.18);
    /// # assert_approx_eq!(v3.y(), 0.18);
    /// # assert_approx_eq!(v3.z(), 0.18);
    /// # assert_approx_eq!(v3.w(), 0.18);
    /// ```  
    #[inline]
    fn mul(self, other: &Vector4) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_mul_ps(self.data, other.data),
            }
        }
    }
}

impl Mul<f32> for &Vector4 {
    type Output = Vector4;

    /// Implements Mul trait for Vector4.
    /// Operator *(Vector4, f32).
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from1(0.1);
    /// let v2 = &v1 * 1.8;
    /// assert_approx_eq!(v2.x(), 0.18);
    /// # assert_approx_eq!(v2.y(), 0.18);
    /// # assert_approx_eq!(v2.z(), 0.18);
    /// # assert_approx_eq!(v2.w(), 0.18);
    /// ```  
    #[inline]
    fn mul(self, other: f32) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_mul_ps(self.data, _mm_set_ps1(other)),
            }
        }
    }
}

impl Div for &Vector4 {
    type Output = Vector4;

    /// Implements Div trait for Vector4.
    /// Operator /(Vector4, Vector4).
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from1(0.1);
    /// let v2 = Vector4::from1(1.8);
    /// let v3 = &v1 / &v2;
    /// assert_approx_eq!(v3.x(), 0.055555556);
    /// # assert_approx_eq!(v3.y(), 0.055555556);
    /// # assert_approx_eq!(v3.z(), 0.055555556);
    /// # assert_approx_eq!(v3.w(), 0.055555556);
    /// ```  
    #[inline]
    fn div(self, other: &Vector4) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_div_ps(self.data, other.data),
            }
        }
    }
}

impl Div<f32> for &Vector4 {
    type Output = Vector4;

    /// Implements Div trait for Vector4.
    /// Operator /(Vector4, f32).
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v1 = Vector4::from1(0.1);
    /// let v2 = &v1 / 1.8;
    /// assert_approx_eq!(v2.x(), 0.055555556);
    /// # assert_approx_eq!(v2.y(), 0.055555556);
    /// # assert_approx_eq!(v2.z(), 0.055555556);
    /// # assert_approx_eq!(v2.w(), 0.055555556);
    /// ```  
    #[inline]
    fn div(self, other: f32) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_div_ps(self.data, _mm_set_ps1(other)),
            }
        }
    }
}

impl Neg for &Vector4 {
    type Output = Vector4;

    /// Implements Neg trait for Vector4.
    /// Operator -(Vector4).
    ///
    /// ```
    /// # use assert_approx_eq::assert_approx_eq;
    /// use rmath_rs::Vector4;
    /// let v = Vector4::from4(0.2, 1.1, -2.9, 99.9);
    /// let v2 = -&v;
    /// assert_approx_eq!(v2.x(), -0.2);
    /// assert_approx_eq!(v2.y(), -1.1);
    /// assert_approx_eq!(v2.z(), 2.9);
    /// assert_approx_eq!(v2.w(), -99.9);
    /// ```  
    #[inline]
    fn neg(self) -> Vector4 {
        unsafe {
            Vector4 {
                data: _mm_xor_ps(self.data, _mm_set_ps1(-0.0)),
            }
        }
    }
}


use std::ops::Neg;



#[derive(Debug,Clone,Copy,PartialEq)]
#[vector_xd_macro::derive_arith]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}


impl Vec3 {
  #[inline(always)]
  pub const fn new(x: f32,y: f32,z: f32)-> Self {
    Vec3 { x,y,z }
  }

  #[inline]
  pub const fn splat(v: f32)-> Self {
    Vec3 {
      x: v,
      y: v,
      z: v
    }
  }

  #[inline]
  pub const fn from_array(v: [f32;3])-> Self {
    Vec3 {
      x: v[0],
      y: v[1],
      z: v[2]
    }
  }

  #[inline]
  pub const fn to_array(self)-> [f32;3] {
    [self.x,self.y,self.z]
  }

  #[inline]
  pub const fn from_slice(v: &[f32])-> Self {
    Vec3 {
      x: v[0],
      y: v[1],
      z: v[2]
    }
  }

  #[inline]
  pub fn write_to_slice(self,slice: &mut [f32]) {
    slice[0]=self.x;
    slice[1]=self.y;
    slice[2]=self.z;
  }
}


impl Neg for Vec3 {
  type Output=Vec3;

  #[inline]
  fn neg(self)-> Self::Output {
    Vec3 {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}




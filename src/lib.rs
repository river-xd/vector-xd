
use std::ops::Neg;



#[derive(Debug,Clone,Copy,PartialEq)]
#[vector_xd_macro::derive_arith]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}


impl Vec3 {
  /// All zeroes.
  pub const ZERO: Self = Self::splat(0.0);

  /// All ones.
  pub const ONE: Self = Self::splat(1.0);

  /// All negative ones.
  pub const NEG_ONE: Self = Self::splat(-1.0);

  /// All `f32::MIN`.
  pub const MIN: Self = Self::splat(f32::MIN);

  /// All `f32::MAX`.
  pub const MAX: Self = Self::splat(f32::MAX);

  /// All `f32::NAN`.
  pub const NAN: Self = Self::splat(f32::NAN);

  /// All `f32::INFINITY`.
  pub const INFINITY: Self = Self::splat(f32::INFINITY);

  /// All `f32::NEG_INFINITY`.
  pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);

  /// A unit vector pointing along the positive X axis.
  pub const X: Self = Self::new(1.0, 0.0, 0.0);

  /// A unit vector pointing along the positive Y axis.
  pub const Y: Self = Self::new(0.0, 1.0, 0.0);

  /// A unit vector pointing along the positive Z axis.
  pub const Z: Self = Self::new(0.0, 0.0, 1.0);

  /// A unit vector pointing along the negative X axis.
  pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);

  /// A unit vector pointing along the negative Y axis.
  pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);

  /// A unit vector pointing along the negative Z axis.
  pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

  /// The unit axes.
  pub const AXES: [Self; 3] = [Self::X, Self::Y, Self::Z];


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

impl Default for Vec3 {
  #[inline(always)]
  fn default()-> Self {
    Self::ZERO
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




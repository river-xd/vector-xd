
use std::{
  mem,
  ops::*,
  fmt::{
    self,
    Debug,
    Display
  },
  iter::{
    Sum,
    Product
  }
};



#[derive(Clone,Copy,PartialEq)]
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


impl AsRef<[f32;3]> for Vec3 {
  #[inline]
  fn as_ref(&self)-> &[f32;3] {
    // SAFETY: trust me bro
    unsafe {
      mem::transmute(self)
    }
  }
}

impl AsMut<[f32;3]> for Vec3 {
  #[inline]
  fn as_mut(&mut self)-> &mut [f32;3] {
    // SAFETY: trust me bro
    unsafe {
      mem::transmute(self)
    }
  }
}


impl Sum for Vec3 {
  #[inline]
  fn sum<I: Iterator<Item=Self>>(iter: I)-> Self {
    iter.fold(Self::ZERO, Self::add)
  }
}

impl<'a> Sum<&'a Self> for Vec3 {
  #[inline]
  fn sum<I:Iterator<Item=&'a Self>>(iter: I)-> Self {
    iter.fold(Self::ZERO, |a, &b| Self::add(a, b))
  }
}

impl Product for Vec3 {
  #[inline]
  fn product<I: Iterator<Item=Self>>(iter: I)-> Self {
    iter.fold(Self::ONE, Self::mul)
  }
}

impl<'a> Product<&'a Self> for Vec3 {
  #[inline]
  fn product<I: Iterator<Item=&'a Self>>(iter: I)-> Self {
    iter.fold(Self::ONE, |a, &b| Self::mul(a, b))
  }
}

impl Index<usize> for Vec3 {
  type Output=f32;
  #[inline]
  fn index(&self, index: usize) -> &Self::Output {
    match index {
      0=> &self.x,
      1=> &self.y,
      2=> &self.z,
      _=> panic!("index out of bounds"),
    }
  }
}

impl IndexMut<usize> for Vec3 {
  #[inline]
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    match index {
      0=> &mut self.x,
      1=> &mut self.y,
      2=> &mut self.z,
      _=> panic!("index out of bounds"),
    }
  }
}



impl Display for Vec3 {
  fn fmt(&self,f: &mut fmt::Formatter<'_>)-> fmt::Result {
    write!(f,"({}, {}, {})",self.x,self.y,self.z)
  }
}


impl Debug for Vec3 {
  fn fmt(&self,fmt: &mut fmt::Formatter<'_>)-> fmt::Result {
    fmt.debug_tuple("Vec3")
    .field(&self.x)
    .field(&self.y)
    .field(&self.z)
    .finish()
  }
}


impl From<[f32;3]> for Vec3 {
  #[inline]
  fn from(arr: [f32;3])-> Self {
    // SAFETY: trust me bro
    unsafe {
      mem::transmute(arr)
    }
  }
}

impl From<Vec3> for [f32;3] {
  #[inline]
  fn from(v: Vec3)-> Self {
    // SAFETY: trust me bro
    unsafe {
      mem::transmute(v)
    }
  }
}

impl From<(f32,f32,f32)> for Vec3 {
  #[inline]
  fn from(touple: (f32,f32,f32))-> Self {
    // SAFETY: trust me bro
    unsafe {
      mem::transmute(touple)
    }
  }
}

impl From<Vec3> for (f32,f32,f32) {
  #[inline]
  fn from(v: Vec3)-> Self {
    // SAFETY: trust me bro
    unsafe {
      mem::transmute(v)
    }
  }
}



















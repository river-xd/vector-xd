
use std::ops::Neg;



#[vector_xd_macro::derive_arith]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32
}


impl Vec3 {
  pub const fn new(x: f32,y: f32,z: f32)-> Self {
    Vec3 { x,y,z }
  }
}


impl Neg for Vec3 {
  type Output=Vec3;

  fn neg(self)-> Self::Output {
    Vec3 {
      x: -self.x,
      y: -self.y,
      z: -self.z
    }
  }
}




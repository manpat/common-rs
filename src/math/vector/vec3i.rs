use crate::math::vector::{Vec3, Vec2i};
use rand_derive2::RandGen;


#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, RandGen, Hash)]
pub struct Vec3i {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}


impl Vec3i {
	pub const fn new(x: i32, y: i32, z: i32) -> Vec3i { Vec3i{x, y, z} }
	pub const fn splat(x: i32) -> Vec3i { Vec3i::new(x, x, x) }
	pub const fn zero() -> Vec3i { Vec3i::splat(0) }
	pub const fn one() -> Vec3i { Vec3i::splat(1) }

	pub fn from_tuple(t: (i32,i32,i32)) -> Vec3i { Vec3i::new(t.0, t.1, t.2) }
	pub fn to_tuple(self) -> (i32,i32,i32) { (self.x, self.y, self.z) }
	pub fn to_array(self) -> [i32; 3] { [self.x, self.y, self.z] }
	pub fn to_vec3(self) -> Vec3 { Vec3::new(self.x as f32, self.y as f32, self.z as f32) }
	pub fn to_xy(&self) -> Vec2i { Vec2i::new(self.x, self.y) }

	pub fn length(self) -> f32 {
		((self.x*self.x + self.y*self.y + self.z*self.z) as f32).sqrt()
	}

	pub fn div_ceil(&self, rhs: Vec3i) -> Vec3i {
		(rhs + *self - Vec3i::splat(1)) / rhs
	}
}


use crate::math::vector::{Vec2, Vec3};
use rand_derive2::RandGen;


#[repr(C)]
#[derive(Copy, Clone, Debug, RandGen)]
pub struct Vec4 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
	pub w: f32,
}

impl Vec4 {
	pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 { Vec4{x, y, z, w} }
	pub const fn splat(x: f32) -> Vec4 { Vec4::new(x, x, x, x) }
	pub const fn zero() -> Vec4 { Vec4::splat(0.0) }
	pub const fn one() -> Vec4 { Vec4::splat(1.0) }

	pub fn from_slice(o: &[f32]) -> Vec4 {
		assert!(o.len() >= 4);
		Vec4::new(o[0], o[1], o[2], o[3])
	}

	pub const fn from_x(x: f32) -> Vec4 { Vec4::new(x, 0.0, 0.0, 0.0) }
	pub const fn from_y(y: f32) -> Vec4 { Vec4::new(0.0, y, 0.0, 0.0) }
	pub const fn from_z(z: f32) -> Vec4 { Vec4::new(0.0, 0.0, z, 0.0) }
	pub const fn from_w(w: f32) -> Vec4 { Vec4::new(0.0, 0.0, 0.0, w) }

	pub fn to_tuple(&self) -> (f32,f32,f32,f32) { (self.x, self.y, self.z, self.w) }
	pub fn to_array(&self) -> [f32; 4] { [self.x, self.y, self.z, self.w] }
	pub fn to_vec3(&self) -> Vec3 { Vec3::new(self.x, self.y, self.z) }

	pub fn to_xyz(&self) -> Vec3 { Vec3::new(self.x, self.y, self.z) }
	pub fn to_xz(&self) -> Vec2 { Vec2::new(self.x, self.z) }
	pub fn to_xy(&self) -> Vec2 { Vec2::new(self.x, self.y) }

	pub fn length(&self) -> f32 { self.dot(*self).sqrt() }

	pub fn dot(&self, o: Vec4) -> f32 { self.x*o.x + self.y*o.y + self.z*o.z + self.w*o.w }
}


impl From<[f32; 4]> for Vec4 {
	fn from([x, y, z, w]: [f32; 4]) -> Vec4 { Vec4{x, y, z, w} }
}

impl From<(f32, f32, f32, f32)> for Vec4 {
	fn from((x, y, z, w): (f32, f32, f32, f32)) -> Vec4 { Vec4{x, y, z, w} }
}



impl From<Vec4> for [f32; 4] {
	fn from(o: Vec4) -> [f32; 4] { o.to_array() }
}

impl From<Vec4> for (f32, f32, f32, f32) {
	fn from(o: Vec4) -> (f32, f32, f32, f32) { o.to_tuple() }
}
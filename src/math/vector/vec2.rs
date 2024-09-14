use crate::math::vector::{Vec2i, Vec3};
use rand_derive2::RandGen;


#[repr(C)]
#[derive(Copy, Clone, Debug, RandGen)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Vec2 {
	pub const fn new(x: f32, y: f32) -> Vec2 { Vec2{x, y} }
	pub const fn splat(x: f32) -> Vec2 { Vec2::new(x, x) }
	pub const fn zero() -> Vec2 { Vec2::splat(0.0) }
	pub const fn one() -> Vec2 { Vec2::splat(1.0) }
	pub fn from_angle(th: f32) -> Vec2 { Vec2::new(th.cos(), th.sin()) }
	pub fn from_slice(o: &[f32]) -> Vec2 {
		assert!(o.len() >= 2);
		Vec2::new(o[0], o[1])
	}

	pub fn from_aspect(aspect: f32) -> Vec2 {
		let aspect = aspect.max(0.001);
		if aspect > 1.0 {
			Vec2::new(aspect, 1.0)
		} else {
			Vec2::new(1.0, 1.0/aspect)
		}
	}

	pub const fn from_x(x: f32) -> Vec2 { Vec2::new(x, 0.0) }
	pub const fn from_y(y: f32) -> Vec2 { Vec2::new(0.0, y) }

	#[deprecated = "Use to_x0y instead. This name makes no sense."]
	pub fn to_x0z(self) -> Vec3 { Vec3::new(self.x, 0.0, self.y) }

	pub fn to_x0(self) -> Vec2 { Vec2::new(self.x, 0.0) }
	pub fn to_0y(self) -> Vec2 { Vec2::new(0.0, self.y) }

	pub fn to_x0y(self) -> Vec3 { Vec3::new(self.x, 0.0, self.y) }
	pub fn to_xny(self, n: f32) -> Vec3 { Vec3::new(self.x, n, self.y) }
	pub fn to_xyn(self, n: f32) -> Vec3 { Vec3::new(self.x, self.y, n) }
	pub fn extend(self, z: f32) -> Vec3 { self.to_xyn(z) }

	pub fn to_vec2i(self) -> Vec2i { Vec2i::new(self.x as i32, self.y as i32) }
	pub fn to_tuple(self) -> (f32,f32) { (self.x, self.y) }
	pub fn to_array(self) -> [f32; 2] { [self.x, self.y] }
	pub fn to_angle(self) -> f32 { self.y.atan2(self.x) }

	pub fn square_length(self) -> f32 { self.dot(self) }
	pub fn length(self) -> f32 { self.dot(self).sqrt() }

	/// CCW 90° rotate
	pub fn perp(self) -> Vec2 { Vec2::new(-self.y, self.x) }

	pub fn normalize(self) -> Vec2 { self * (1.0/self.length()) }
	pub fn dot(self, o: Vec2) -> f32 { self.x*o.x + self.y*o.y }
	pub fn wedge(self, o: Vec2) -> f32 { self.x*o.y - self.y*o.x }

	pub fn floor(self) -> Vec2 { Vec2::new(self.x.floor(), self.y.floor()) }
	pub fn ceil(self) -> Vec2 { Vec2::new(self.x.ceil(), self.y.ceil()) }

	/// Swaps x and y elements.
	pub fn transpose(self) -> Vec2 {
		Vec2::new(self.y, self.x)
	}
}




pub trait ToVec2Scalar {
	fn to_vec2(&self) -> Vec2;
}

impl ToVec2Scalar for Vec2 {
	fn to_vec2(&self) -> Vec2 { *self }
}

impl ToVec2Scalar for f32 {
	fn to_vec2(&self) -> Vec2 { Vec2::splat(*self) }
}
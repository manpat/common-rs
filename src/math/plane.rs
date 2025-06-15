use crate::vector::{Vec2, Vec3, Vec4};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Plane3 {
	pub normal: Vec3,
	pub distance: f32,
}

impl Plane3 {
	/// A degenerate plane at infinity. It is 1 unit behind from every point in space.
	pub const NEGATIVE_INFINITY: Plane3 = Plane3 {
		normal: Vec3::zero(),
		distance: -1.0
	};

	pub fn new(n: Vec3, distance: f32) -> Self {
		Plane3 {normal: n.normalize(), distance}
	}

	pub fn from_points(a: Vec3, b: Vec3, c: Vec3) -> Self {
		let ab = (b-a).normalize();
		let ac = (c-a).normalize();
		let normal = ab.cross(ac).normalize();
		Plane3 {
			normal,
			distance: normal.dot(a),
		}
	}

	pub fn to_vec4(&self) -> Vec4 {
		self.normal.extend(self.distance)
	}

	pub fn distance_to(&self, p: Vec3) -> f32 {
		self.normal.dot(p) - self.distance
	}

	pub fn project(&self, p: Vec3) -> Vec3 {
		p - self.normal * self.distance_to(p)
	}

	pub fn mirror(&self, p: Vec3) -> Vec3 {
		p - self.normal * self.distance_to(p) * 2.0
	}
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Plane2 {
	pub normal: Vec2,
	pub distance: f32,
}

impl Plane2 {
	/// A degenerate plane at infinity. It is 1 unit behind from every point in space.
	pub const NEGATIVE_INFINITY: Plane2 = Plane2 {
		normal: Vec2::zero(),
		distance: -1.0
	};

	pub fn new(n: Vec2, distance: f32) -> Self {
		Plane2 {normal: n.normalize(), distance}
	}

	pub fn from_points(a: Vec2, b: Vec2) -> Self {
		let ab = (b-a).normalize();
		let normal = ab.perp();

		Plane2 {
			normal,
			distance: normal.dot(a),
		}
	}

	pub fn to_x0y(&self) -> Plane3 {
		Plane3 {
			normal: self.normal.to_x0y(),
			distance: self.distance,
		}
	}

	pub fn to_vec3(&self) -> Vec3 {
		self.normal.extend(self.distance)
	}

	pub fn distance_to(&self, p: Vec2) -> f32 {
		self.normal.dot(p) - self.distance
	}

	pub fn project(&self, p: Vec2) -> Vec2 {
		p - self.normal * self.distance_to(p)
	}

	pub fn mirror(&self, p: Vec2) -> Vec2 {
		p - self.normal * self.distance_to(p) * 2.0
	}
}

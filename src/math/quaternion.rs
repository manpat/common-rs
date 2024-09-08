use std::ops::{Add, Mul};
use crate::matrix::*;
use crate::vector::*;
use crate::lerp::Lerp;
use std::f32::consts::PI;

#[derive(Copy, Clone, Debug)]
pub struct Quat {
	pub imaginary: Vec3,
	pub real: f32,
}

impl Quat {
	pub const fn new(real: f32, imaginary: Vec3) -> Quat {
		Quat{real, imaginary}
	}

	pub const fn from_imaginary(imaginary: Vec3) -> Quat {
		Quat::new(0.0, imaginary)
	}

	pub const fn identity() -> Quat {
		Quat::new(1.0, Vec3::zero())
	}

	pub fn from_axis_angle(axis: Vec3, angle: f32) -> Quat {
		let angle = angle / 2.0;

		Quat::new(
			angle.cos(),
			axis * angle.sin(),
		)
	}

	pub fn from_pitch(pitch: f32) -> Quat {
		Quat::from_axis_angle(Vec3::from_x(1.0), pitch)
	}

	pub fn from_yaw(yaw: f32) -> Quat {
		Quat::from_axis_angle(Vec3::from_y(1.0), yaw)
	}

	pub fn from_roll(roll: f32) -> Quat {
		Quat::from_axis_angle(Vec3::from_z(1.0), roll)
	}

	pub fn forward(&self) -> Vec3 { -self.backward() }
	pub fn backward(&self) -> Vec3 { *self * Vec3::from_z(1.0) }
	pub fn right(&self) -> Vec3 { *self * Vec3::from_x(1.0) }
	pub fn up(&self) -> Vec3 { *self * Vec3::from_y(1.0) }

	pub fn magnitude(&self) -> f32 {
		(self.imaginary.square_length() + self.real*self.real).sqrt()
	}

	pub fn normalize(&self) -> Quat {
		let m = self.magnitude();
		Quat::new(self.real/m, self.imaginary/m)
	}

	pub fn conjugate(&self) -> Quat {
		Quat::new(self.real, -self.imaginary)
	}

	pub fn scale(&self, f: f32) -> Quat {
		// TODO: improve
		(*self * f + Quat::identity() * (1.0 - f)).normalize()
	}

	pub fn to_mat4(&self) -> Mat4 {
		self.to_mat3x4().to_mat4()
	}

	pub fn to_mat3x4(&self) -> Mat3x4 {
		// TODO(pat.m): this could be _much_ more efficient
		Mat3x4::from_columns([
			self.right(),
			self.up(),
			self.backward(),
			Vec3::zero(),
		])
	}

	// Stolen and adjusted from https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles
	// TODO: test these!
	pub fn yaw(&self) -> f32 {
		let Vec3{x, y, z} = self.imaginary;
		let siny_cosp = 2.0 * (self.real * y + z * x);
		let cosy_cosp = 1.0 - 2.0 * (x * x + y * y);
		siny_cosp.atan2(cosy_cosp)
	}

	pub fn roll(&self) -> f32 {
		let Vec3{x, y, z} = self.imaginary;
		let sinr_cosp = 2.0 * (self.real * z + x * y);
		let cosr_cosp = 1.0 - 2.0 * (x * x + z * z);
		sinr_cosp.atan2(cosr_cosp)
	}

	pub fn pitch(&self) -> f32 {
		let Vec3{x, y, z} = self.imaginary;
		let sinp = 2.0 * (self.real * x - y * z);
		if sinp.abs() >= 1.0 {
			(PI / 2.0).copysign(sinp) // use 90 degrees if out of range
		} else {
			sinp.asin()
		}
	}
}


impl Add<Quat> for Quat {
	type Output = Quat;
	fn add(self, o: Quat) -> Quat {
		Quat::new(self.real+o.real, self.imaginary+o.imaginary)
	}
}


impl Mul<Quat> for Quat {
	type Output = Quat;
	fn mul(self, o: Quat) -> Quat {
		// (s, u) (t, v) = (st - u.v, sv + tu + u x v)
		Quat::new(
			self.real*o.real - self.imaginary.dot(o.imaginary),
			self.real * o.imaginary + o.real * self.imaginary + self.imaginary.cross(o.imaginary),
		)
	}
}

impl Mul<f32> for Quat {
	type Output = Quat;
	fn mul(self, o: f32) -> Quat {
		Quat::new(self.real*o, self.imaginary*o)
	}
}

// TODO(pat.m): maybe this shouldn't be on Mul, but just a regular function
impl Mul<Vec3> for Quat {
	type Output = Vec3;
	fn mul(self, v: Vec3) -> Vec3 {
		// TODO(pat.m): this can be simplified further

		// This is a simplified form of (self * Quat::from_imaginary(v) * self.conjugate()).imaginary.
		// This takes advantage of the zero real component of `v`, and the fact that
		// the real component after multiplication by the conjugate will always be zero.
		// NOTE: This assumes a unit quaternion.
		let half_rotated = multiply_quat_vec3(&self, v);
		quat_multiply_conjugate_discarding_real(&half_rotated, &self)
	}
}


// Implements q * (0, v)
fn multiply_quat_vec3(q: &Quat, v: Vec3) -> Quat {
	// (s, u) (0, v) = (-u.v, sv + u x v)
	Quat {
		real: -q.imaginary.dot(v),
		imaginary: q.real * v + q.imaginary.cross(v),
	}
}

// Implements imaginary(a * conjugate(b))
fn quat_multiply_conjugate_discarding_real(a: &Quat, b: &Quat) -> Vec3 {
	// (r, u) (s, v) = (rs - u.v, rv + su + u x v)
	// im((r, u) (s, -v)) = rv - su - u x v
	b.real * a.imaginary - a.real * b.imaginary + a.imaginary.cross(-b.imaginary)
}


// TODO(pat.m): slerp?
impl Lerp<Quat> for f32 {
	fn lerp(self, start: Quat, end: Quat) -> Quat {
		Quat {
			imaginary: self.lerp(start.imaginary, end.imaginary),
			real: self.lerp(start.real, end.real),
		}
	}
}


#[cfg(feature = "serde")]
impl serde::Serialize for Quat {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where S: serde::Serializer
	{
		let Vec3{x, y, z} = self.imaginary;
		[x, y, z, self.real].serialize(serializer)
	}
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Quat {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where D: serde::Deserializer<'de>
	{
		<[f32; 4]>::deserialize(deserializer)
			.map(|[x, y, z, real]| Quat::new(real, Vec3::new(x, y, z)))
	}
}


#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn test_from_pitch() {
		let ident = Quat::from_pitch(0.0);
		assert_vec_eq!(ident.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(ident.right(), Vec3::from_x(1.0));
		assert_vec_eq!(ident.up(), Vec3::from_y(1.0));
		assert_almost_eq!(ident.yaw(), 0.0);
		assert_almost_eq!(ident.pitch(), 0.0);
		assert_almost_eq!(ident.roll(), 0.0);

		let r90 = Quat::from_pitch(PI/2.0);
		assert_vec_eq!(r90.forward(), Vec3::from_y(1.0));
		assert_vec_eq!(r90.right(), Vec3::from_x(1.0));
		assert_vec_eq!(r90.up(), Vec3::from_z(1.0));
		assert_almost_eq!(r90.yaw(), 0.0);
		assert_almost_eq!(r90.pitch(), PI/2.0);
		assert_almost_eq!(r90.roll(), 0.0);

		let r180 = Quat::from_pitch(PI);
		assert_vec_eq!(r180.forward(), Vec3::from_z(1.0));
		assert_vec_eq!(r180.right(), Vec3::from_x(1.0));
		assert_vec_eq!(r180.up(), Vec3::from_y(-1.0));
		// Yay for angle stability
		// assert_almost_eq!(r180.yaw(), 0.0);
		// assert_almost_eq!(r180.pitch(), PI);
		// assert_almost_eq!(r180.roll(), 0.0);
	}

	#[test]
	fn test_from_yaw() {
		let ident = Quat::from_yaw(0.0);
		assert_vec_eq!(ident.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(ident.right(), Vec3::from_x(1.0));
		assert_vec_eq!(ident.up(), Vec3::from_y(1.0));
		assert_almost_eq!(ident.yaw(), 0.0);
		assert_almost_eq!(ident.pitch(), 0.0);
		assert_almost_eq!(ident.roll(), 0.0);

		let r45 = Quat::from_yaw(PI/4.0);
		assert_vec_eq!(r45.forward(), Vec3::new(-INV_SQRT_2, 0.0, -INV_SQRT_2));
		assert_vec_eq!(r45.right(), Vec3::new(INV_SQRT_2, 0.0, -INV_SQRT_2));
		assert_vec_eq!(r45.up(), Vec3::from_y(1.0));
		assert_almost_eq!(r45.yaw(), PI/4.0);
		assert_almost_eq!(r45.pitch(), 0.0);
		assert_almost_eq!(r45.roll(), 0.0);

		let r90 = Quat::from_yaw(PI/2.0);
		assert_vec_eq!(r90.forward(), Vec3::from_x(-1.0));
		assert_vec_eq!(r90.right(), Vec3::from_z(-1.0));
		assert_vec_eq!(r90.up(), Vec3::from_y(1.0));
		assert_almost_eq!(r90.yaw(), PI/2.0);
		assert_almost_eq!(r90.pitch(), 0.0);
		assert_almost_eq!(r90.roll(), 0.0);

		let r180 = Quat::from_yaw(PI);
		assert_vec_eq!(r180.forward(), Vec3::from_z(1.0));
		assert_vec_eq!(r180.right(), Vec3::from_x(-1.0));
		assert_vec_eq!(r180.up(), Vec3::from_y(1.0));
		// Yay for angle stability
		// assert_almost_eq!(r180.yaw(), PI);
		// assert_almost_eq!(r180.pitch(), 0.0);
		// assert_almost_eq!(r180.roll(), 0.0);
	}

	#[test]
	fn test_from_roll() {
		let ident = Quat::from_roll(0.0);
		assert_vec_eq!(ident.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(ident.right(), Vec3::from_x(1.0));
		assert_vec_eq!(ident.up(), Vec3::from_y(1.0));
		assert_almost_eq!(ident.yaw(), 0.0);
		assert_almost_eq!(ident.pitch(), 0.0);
		assert_almost_eq!(ident.roll(), 0.0);

		let r90 = Quat::from_roll(PI/2.0);
		assert_vec_eq!(r90.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(r90.right(), Vec3::from_y(1.0));
		assert_vec_eq!(r90.up(), Vec3::from_x(-1.0));
		assert_almost_eq!(r90.yaw(), 0.0);
		assert_almost_eq!(r90.pitch(), 0.0);
		assert_almost_eq!(r90.roll(), PI/2.0);

		let r180 = Quat::from_roll(PI);
		assert_vec_eq!(r180.forward(), Vec3::from_z(-1.0));
		assert_vec_eq!(r180.right(), Vec3::from_x(-1.0));
		assert_vec_eq!(r180.up(), Vec3::from_y(-1.0));
		// Yay for angle stability
		// assert_almost_eq!(r180.yaw(), 0.0);
		// assert_almost_eq!(r180.pitch(), 0.0);
		// assert_almost_eq!(r180.roll(), PI);
	}
}

use crate::math::*;

/// A Half Open 2D Range.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aabb2i {
	/// The inclusive lower bound of the range.
	pub lower: Vec2i,

	/// The exclusive upper bound of the range.
	pub upper: Vec2i,
}

/// Constructors & Conversions
impl Aabb2i {
	pub fn new(lower: Vec2i, upper: Vec2i) -> Aabb2i {
		Aabb2i { lower, upper }
	}

	pub fn empty() -> Aabb2i {
		Aabb2i::new(
			Vec2i::splat(i32::MAX),
			Vec2i::splat(i32::MIN)
		)
	}

	pub fn around_point(center: Vec2i, extents: Vec2i) -> Aabb2i {
		Aabb2i::new(center - extents, center + extents)
	}

	pub fn to_aabb2(&self) -> Aabb2 {
		if self.is_empty() {
			Aabb2::empty()
		} else {
			Aabb2 {
				min: self.lower.to_vec2(),
				max: self.upper.to_vec2(), // This is a little weird
			}
		}
	}
}

/// Properties
impl Aabb2i {
	pub fn is_empty(&self) -> bool {
		self.lower.x >= self.upper.x
		|| self.lower.y >= self.upper.y
	}

	pub fn center(&self) -> Vec2i {
		(self.lower + self.upper) / 2
	}

	pub fn size(&self) -> Vec2i {
		if self.is_empty() {
			Vec2i::zero()
		} else {
			self.upper - self.lower
		}
	}

	pub fn extents(&self) -> Vec2i {
		self.size() / 2
	}

	pub fn aspect(&self) -> f32 {
		self.to_aabb2().aspect()
	}
}


/// Queries
impl Aabb2i {
	pub fn contains_point(&self, point: Vec2i) -> bool {
		self.lower.x <= point.x && point.x < self.upper.x
		&& self.lower.y <= point.y && point.y < self.upper.y
	}
}

/// Modifications
impl Aabb2i {
	pub fn grow(&self, amount: Vec2i) -> Self {
		Aabb2i {
			lower: self.lower - amount,
			upper: self.upper + amount,
		}
	}

	pub fn shrink(&self, amount: Vec2i) -> Self {
		self.grow(-amount)
	}

	// TODO(pat.m): include or exclude upper bound point?
	pub fn include_point(&self, point: Vec2i) -> Self {
		Aabb2i {
			lower: Vec2i::new(self.lower.x.min(point.x), self.lower.y.min(point.y)),
			upper: Vec2i::new(self.upper.x.max(point.x), self.upper.y.max(point.y)),
		}
	}
}


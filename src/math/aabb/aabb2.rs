use crate::math::vector::Vec2;

/// A Closed 2D Range - that is min and max count as being inside the bounds of the Aabb2
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Aabb2 {
	pub min: Vec2,
	pub max: Vec2,
}

/// Constructors
impl Aabb2 {
	pub fn new(min: Vec2, max: Vec2) -> Aabb2 {
		Aabb2 { min, max }
	}

	#[deprecated="Use Aabb2::empty instead"]
	pub fn new_empty() -> Aabb2 {
		Self::empty()
	}

	pub fn empty() -> Aabb2 {
		Aabb2::new(
			Vec2::splat(f32::INFINITY),
			Vec2::splat(-f32::INFINITY)
		)
	}

	pub fn zero() -> Aabb2 {
		Aabb2::point(Vec2::zero())
	}

	pub fn point(center: Vec2) -> Aabb2 {
		Aabb2::new(center, center)
	}

	pub fn around_point(center: Vec2, extents: Vec2) -> Aabb2 {
		Aabb2::new(center - extents, center + extents)
	}
}

/// Properties
impl Aabb2 {
	pub fn is_empty(&self) -> bool {
		self.min.x > self.max.x
		|| self.min.y > self.max.y
	}

	pub fn center(&self) -> Vec2 {
		(self.min + self.max) / 2.0
	}

	pub fn size(&self) -> Vec2 {
		if self.is_empty() {
			Vec2::zero()
		} else {
			self.max - self.min
		}
	}

	pub fn extents(&self) -> Vec2 {
		self.size() / 2.0
	}

	pub fn aspect(&self) -> f32 {
		let Vec2{x, y} = self.size();
		x / y.max(0.0001)
	}

	pub fn min_max_corner(&self) -> Vec2 {
		Vec2 {
			x: self.min.x,
			y: self.max.y,
		}
	}

	pub fn max_min_corner(&self) -> Vec2 {
		Vec2 {
			x: self.max.x,
			y: self.min.y,
		}
	}
}


/// Queries
impl Aabb2 {
	pub fn contains_point(&self, point: Vec2) -> bool {
		self.min.x <= point.x && point.x <= self.max.x
		&& self.min.y <= point.y && point.y <= self.max.y
	}
}

/// Modifications
impl Aabb2 {
	pub fn grow(&self, amount: Vec2) -> Self {
		Aabb2 {
			min: self.min - amount,
			max: self.max + amount,
		}
	}

	pub fn shrink(&self, amount: Vec2) -> Self {
		self.grow(-amount)
	}

	pub fn translate(&self, amount: Vec2) -> Self {
		Aabb2 {
			min: self.min + amount,
			max: self.max + amount,
		}
	}

	pub fn expand_to_include_rect(&self, other: Aabb2) -> Self {
		Aabb2 {
			min: Vec2::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
			max: Vec2::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
		}
	}
}




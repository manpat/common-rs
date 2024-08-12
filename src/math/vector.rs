use std::ops::{Add, Sub, Mul, Div, Neg};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
use std::iter::{Sum, Product};
use crate::lerp::Lerp;

pub mod vec2;
pub mod vec3;
pub mod vec4;
pub mod vec2i;
pub mod vec3i;

pub mod map;

pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
pub use vec2i::*;
pub use vec3i::*;


macro_rules! impl_vector_bin_op {
	($ty:ident, $trait:ident<$scalar:ty>, $fn:ident, $op:tt, $($els:ident),+) => {
		impl $trait for $ty {
			type Output = $ty;
			fn $fn(self, o: $ty) -> $ty {
				$ty::new($(self.$els $op o.$els),+)
			}
		}

		impl $trait<$scalar> for $ty {
			type Output = $ty;
			fn $fn(self, o: $scalar) -> $ty {
				$ty::new($(self.$els $op o),+)
			}
		}

		impl $trait<$ty> for $scalar {
			type Output = $ty;
			fn $fn(self, o: $ty) -> $ty {
				$ty::new($(self $op o.$els),+)
			}
		}
	};

	(ass $ty:ident, $trait:ident<$scalar:ty>, $fn:ident, $op:tt, $($els:ident),+) => {
		impl $trait for $ty {
			fn $fn(&mut self, o: $ty) {
				$(
					self.$els $op o.$els;
				)+
			}
		}

		impl $trait<$scalar> for $ty {
			fn $fn(&mut self, o: $scalar) {
				$(
					self.$els $op o;
				)+
			}
		}
	};
}


macro_rules! repeat_for_each {
    ( $each:tt, $with:tt ) => { $with };
}

macro_rules! bulk_impl_vector_ops {
	($ty:ident { $($els:ident),+ } => [ $scalar:ty ; $size:expr ]) => {
		impl_vector_bin_op!($ty, Add<$scalar>, add, +, $($els),+);
		impl_vector_bin_op!($ty, Sub<$scalar>, sub, -, $($els),+);
		impl_vector_bin_op!($ty, Mul<$scalar>, mul, *, $($els),+);
		impl_vector_bin_op!($ty, Div<$scalar>, div, /, $($els),+);

		impl_vector_bin_op!(ass $ty, AddAssign<$scalar>, add_assign, +=, $($els),+);
		impl_vector_bin_op!(ass $ty, SubAssign<$scalar>, sub_assign, -=, $($els),+);
		impl_vector_bin_op!(ass $ty, MulAssign<$scalar>, mul_assign, *=, $($els),+);
		impl_vector_bin_op!(ass $ty, DivAssign<$scalar>, div_assign, /=, $($els),+);

		impl Default for $ty {
			fn default() -> $ty {
				$ty::zero()
			}
		}

		impl Neg for $ty {
			type Output = $ty;
			fn neg(self) -> $ty {
				$ty::new($(-self.$els),+)
			}
		}

		impl Sum for $ty {
			fn sum<I>(iter: I) -> $ty where I: Iterator<Item=$ty> {
				iter.fold($ty::zero(), |a, v| a + v)
			}
		}
		
		impl<'a> Sum<&'a $ty> for $ty {
			fn sum<I>(iter: I) -> $ty where I: Iterator<Item=&'a $ty> {
				iter.fold($ty::zero(), |a, &v| a + v)
			}
		}

		impl Product for $ty {
			fn product<I>(iter: I) -> $ty where I: Iterator<Item=$ty> {
				iter.fold($ty::splat(1 as $scalar), |a, v| a * v)
			}
		}

		impl<'a> Product<&'a $ty> for $ty {
			fn product<I>(iter: I) -> $ty where I: Iterator<Item=&'a $ty> {
				iter.fold($ty::splat(1 as $scalar), |a, &v| a * v)
			}
		}
		
		#[cfg(feature = "serde")]
		impl serde::Serialize for $ty {
			fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
				where S: serde::Serializer
			{
				[$(self.$els),+].serialize(serializer)
			}
		}

		#[cfg(feature = "serde")]
		impl<'de> serde::Deserialize<'de> for $ty {
			fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where D: serde::Deserializer<'de>
			{
				<[$scalar; $size]>::deserialize(deserializer)
					.map($ty::from)
			}
		}

		// Array conversions
		impl From<$ty> for [$scalar; $size] {
			fn from($ty { $($els),+ }: $ty) -> Self {
				[$($els),+]
			}
		}

		impl From<[$scalar; $size]> for $ty {
			fn from([$($els),+]: [$scalar; $size]) -> Self {
				$ty{ $($els),+ }
			}
		}

		// Tuple conversions
		impl From<$ty> for ( $(repeat_for_each!($els, $scalar)),+ ) {
			fn from($ty { $($els),+ }: $ty) -> Self {
				($($els),+)
			}
		}

		impl From<( $(repeat_for_each!($els, $scalar)),+ )> for $ty {
			fn from(($($els),+): ( $(repeat_for_each!($els, $scalar)),+ )) -> Self {
				$ty { $($els),+ }
			}
		}

		// Array compatible conversions
		impl $ty {
			pub fn to_compatible<T>(&self) -> T
				where T: From<[$scalar; $size]>
			{
				<[$scalar; $size]>::from(*self).into()
			}

			pub fn from_compatible(o: impl Into<[$scalar; $size]>) -> Self {
				o.into().into()
			}
		}

		// Reference conversions
		impl AsRef<[$scalar; $size]> for $ty {
			fn as_ref(&self) -> &[$scalar; $size] {
				// SAFETY: repr(C) guarantees the layout is compatible
				unsafe { std::mem::transmute(self) }
			}
		}

		impl AsMut<[$scalar; $size]> for $ty {
			fn as_mut(&mut self) -> &mut [$scalar; $size] {
				// SAFETY: repr(C) guarantees the layout is compatible
				unsafe { std::mem::transmute(self) }
			}
		}
	};
}

bulk_impl_vector_ops!(Vec2 { x, y }       => [f32; 2]);
bulk_impl_vector_ops!(Vec3 { x, y, z }    => [f32; 3]);
bulk_impl_vector_ops!(Vec4 { x, y, z, w } => [f32; 4]);
bulk_impl_vector_ops!(Vec2i { x, y }      => [i32; 2]);
bulk_impl_vector_ops!(Vec3i { x, y, z }   => [i32; 3]);

macro_rules! impl_lerp_for_vec {
	($ty:ident, $($els:ident),+) => (
		impl Lerp<$ty> for f32 {
			fn lerp(self, start: $ty, end: $ty) -> $ty {
				$ty {
					$($els: self.lerp(start.$els, end.$els)),+
				}
			}
		}
	);
}

impl_lerp_for_vec!(Vec2, x, y);
impl_lerp_for_vec!(Vec3, x, y, z);
impl_lerp_for_vec!(Vec4, x, y, z, w);


// mint interop
macro_rules! impl_mint_intomint {
	($ty:ty, $mint_ty:ty) => {
		#[cfg(feature="interop")]
		impl mint::IntoMint for $ty {
			type MintType = $mint_ty;
		}

		#[cfg(feature="interop")]
		impl $ty {
			pub fn from_mint(o: impl Into<$mint_ty>) -> $ty { o.into().into() }
			pub fn to_mint(self) -> $mint_ty { self.into() }
		}
	}
}

macro_rules! impl_mint_interop {
	($ty:ty, $mint_ty:ty, $array:ty) => {
		#[cfg(feature="interop")]
		impl From<$mint_ty> for $ty {
			fn from(o: $mint_ty) -> Self {
				<$array>::from(o).into()
			}
		}

		#[cfg(feature="interop")]
		impl From<$ty> for $mint_ty {
			fn from(o: $ty) -> Self {
				<$array>::from(o).into()
			}
		}
	}
}

impl_mint_intomint!(Vec2, mint::Vector2<f32>);
impl_mint_intomint!(Vec3, mint::Vector3<f32>);
impl_mint_intomint!(Vec4, mint::Vector4<f32>);
impl_mint_intomint!(Vec2i, mint::Vector2<i32>);
impl_mint_intomint!(Vec3i, mint::Vector3<i32>);

impl_mint_interop!(Vec2, mint::Vector2<f32>, [f32; 2]);
impl_mint_interop!(Vec3, mint::Vector3<f32>, [f32; 3]);
impl_mint_interop!(Vec4, mint::Vector4<f32>, [f32; 4]);
impl_mint_interop!(Vec2i, mint::Vector2<i32>, [i32; 2]);
impl_mint_interop!(Vec3i, mint::Vector3<i32>, [i32; 3]);

impl_mint_interop!(Vec2, mint::Point2<f32>, [f32; 2]);
impl_mint_interop!(Vec3, mint::Point3<f32>, [f32; 3]);
impl_mint_interop!(Vec2i, mint::Point2<i32>, [i32; 2]);
impl_mint_interop!(Vec3i, mint::Point3<i32>, [i32; 3]);
use super::*;

use cint::{PremultipliedAlpha, Alpha, LinearSrgb, EncodedSrgb, ColorInterop};

impl ColorInterop for Color {
	type CintTy = Alpha<LinearSrgb<f32>>;
}

// To cint
impl From<Alpha<LinearSrgb<f32>>> for Color {
	fn from(o: Alpha<LinearSrgb<f32>>) -> Color { Color::from(*o.as_ref()) }
}

impl From<Alpha<EncodedSrgb<u8>>> for Color {
	fn from(o: Alpha<EncodedSrgb<u8>>) -> Color { Color::from(*o.as_ref()).to_linear() }
}
impl From<PremultipliedAlpha<LinearSrgb<f32>>> for Color {
	fn from(o: PremultipliedAlpha<LinearSrgb<f32>>) -> Color { Color::from(*o.as_ref()).to_unmultiplied() }
}

impl From<PremultipliedAlpha<EncodedSrgb<u8>>> for Color {
	fn from(o: PremultipliedAlpha<EncodedSrgb<u8>>) -> Color { Color::from(*o.as_ref()).to_unmultiplied().to_linear() }
}

impl From<LinearSrgb<f32>> for Color {
	fn from(o: LinearSrgb<f32>) -> Color { Color::from(*o.as_ref()) }
}

impl From<EncodedSrgb<u8>> for Color {
	fn from(o: EncodedSrgb<u8>) -> Color { Color::from(*o.as_ref()).to_linear() }
}


// From cint
impl From<Color> for Alpha<LinearSrgb<f32>> {
	fn from(o: Color) -> Self { o.to_array().into() }
}

impl From<Color> for Alpha<EncodedSrgb<u8>> {
	fn from(o: Color) -> Self { o.to_srgb().to_byte_array().into() }
}

impl From<Color> for PremultipliedAlpha<LinearSrgb<f32>> {
	fn from(o: Color) -> Self { o.to_premultiplied().to_array().into() }
}

impl From<Color> for PremultipliedAlpha<EncodedSrgb<u8>> {
	fn from(o: Color) -> Self { o.to_srgb().to_premultiplied().to_byte_array().into() }
}

impl From<Color> for LinearSrgb<f32> {
	fn from(o: Color) -> Self { <[f32; 3]>::from(o).into() }
}

impl From<Color> for EncodedSrgb<u8> {
	fn from(o: Color) -> Self { <[u8; 3]>::from(o.to_srgb()).into() }
}


// mint interop
impl mint::IntoMint for Color {
	type MintType = mint::Vector4<f32>;
}

impl From<mint::Vector4<f32>> for Color {
	fn from(o: mint::Vector4<f32>) -> Self {
		Self::from_slice(o.as_ref())
	}
}

impl From<mint::Vector3<f32>> for Color {
	fn from(o: mint::Vector3<f32>) -> Self {
		Self::from_slice(o.as_ref())
	}
}

impl From<Color> for mint::Vector4<f32> {
	fn from(o: Color) -> Self {
		Self::from(<[f32; 4]>::from(o))
	}
}

impl From<Color> for mint::Vector3<f32> {
	fn from(o: Color) -> Self {
		Self::from(<[f32; 3]>::from(o))
	}
}
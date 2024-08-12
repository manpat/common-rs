use super::*;

use cint::{PremultipliedAlpha, Alpha, LinearSrgb, EncodedSrgb, ColorType, ColorInterop};


impl ColorInterop for Color {
	type CintTy = Alpha<LinearSrgb<f32>>;
}

// To cint
impl From<Alpha<LinearSrgb<f32>>> for Color {
	fn from(o: Alpha<LinearSrgb<f32>>) -> Color { Color::from(*o.as_ref()) }
}

impl From<Alpha<EncodedSrgb<u8>>> for Color {
	fn from(o: Alpha<EncodedSrgb<u8>>) -> Color { Color::from(*o.as_ref()) }
}

impl From<LinearSrgb<f32>> for Color {
	fn from(o: LinearSrgb<f32>) -> Color { Color::from(*o.as_ref()) }
}

impl From<EncodedSrgb<u8>> for Color {
	fn from(o: EncodedSrgb<u8>) -> Color { Color::from(*o.as_ref()) }
}


// From cint
impl From<Color> for Alpha<LinearSrgb<f32>> {
	fn from(o: Color) -> Self { o.to_array().into() }
}

impl From<Color> for Alpha<EncodedSrgb<u8>> {
	fn from(o: Color) -> Self { o.to_byte_array().into() }
}

impl From<Color> for LinearSrgb<f32> {
	fn from(o: Color) -> Self { <[f32; 3]>::from(o).into() }
}

impl From<Color> for EncodedSrgb<u8> {
	fn from(o: Color) -> Self { <[u8; 3]>::from(o).into() }
}


// Deal with premultiplied alpha conversions
impl<T> From<Color> for PremultipliedAlpha<T>
	where PremultipliedAlpha<T>: From<[f32; 4]>
		, T: ColorType
{
	fn from(Color{r, g, b, a}: Color) -> Self {
		Self::from([r*a, b*a, g*a, a])
	}
}

impl<T> From<PremultipliedAlpha<T>> for Color
	where PremultipliedAlpha<T>: Into<[f32; 4]>
		, T: ColorType
{
	fn from(o: PremultipliedAlpha<T>) -> Self {
		let [r, g, b, a] = o.into();

		if a > 0.0 {
			Self::from([r/a, b/a, g/a, a])
		} else {
			Self::from([0.0, 0.0, 0.0, a])
		}
	}
}
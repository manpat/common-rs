use crate::math::vector::*;
use crate::math::Lerp;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
	pub r: f32,
	pub g: f32,
	pub b: f32,
	pub a: f32,
}

#[allow(dead_code)]
impl Color {
	pub const fn rgba(r:f32, g:f32, b:f32, a:f32) -> Color {
		Color {r,g,b,a}
	}
	pub const fn rgb(r:f32, g:f32, b:f32) -> Color {
		Color::rgba(r,g,b, 1.0)
	}

	pub const fn rgba8(r:u8, g:u8, b:u8, a:u8) -> Color {
		Color {
			r: r as f32 / 255.0,
			g: g as f32 / 255.0,
			b: b as f32 / 255.0,
			a: a as f32 / 255.0,
		}
	}
	pub const fn rgb8(r:u8, g:u8, b:u8) -> Color {
		Color::rgba8(r,g,b, 255)
	}

	pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> Color {
		let h = h % 360.0 - h.signum().min(0.0) * 360.0;
		// if h < 0.0, add 360.0

		let s = s.clamp(0.0, 1.0);
		let v = v.clamp(0.0, 1.0);

		let c = v * s;
		let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
		let m = v - c;

		let seg = (h / 60.0) as u32 % 6;
		let (r,g,b) = match seg {
			0 => (c, x, 0.0),
			1 => (x, c, 0.0),
			2 => (0.0, c, x),
			3 => (0.0, x, c),
			4 => (x, 0.0, c),
			5 => (c, 0.0, x),
			_ => return Color::black()
		};

		Color::rgba(r+m, g+m, b+m, a)
	}

	pub fn hsv(h: f32, s: f32, v: f32) -> Color {
		Color::hsva(h,s,v, 1.0)
	}

	pub const fn grey(v: f32) -> Color { Color::rgb(v, v, v) }
	pub const fn grey_a(v: f32, a: f32) -> Color { Color::rgba(v, v, v, a) }
	pub const fn white() -> Color { Color::grey(1.0) }
	pub const fn black() -> Color { Color::grey(0.0) }

	pub const fn red() -> Color { Color::rgb(1.0, 0.0, 0.0) }
	pub const fn green() -> Color { Color::rgb(0.0, 1.0, 0.0) }
	pub const fn blue() -> Color { Color::rgb(0.0, 0.0, 1.0) }

	pub const fn yellow() -> Color { Color::rgb(1.0, 1.0, 0.0) }
	pub const fn cyan() -> Color { Color::rgb(0.0, 1.0, 1.0) }
	pub const fn magenta() -> Color { Color::rgb(1.0, 0.0, 1.0) }

	pub const fn light_red() -> Color { Color::rgb(1.0, 0.5, 0.5) }
	pub const fn light_green() -> Color { Color::rgb(0.5, 1.0, 0.5) }
	pub const fn light_blue() -> Color { Color::rgb(0.5, 0.5, 1.0) }

	pub const fn light_yellow() -> Color { Color::rgb(1.0, 1.0, 0.5) }
	pub const fn light_cyan() -> Color { Color::rgb(0.5, 1.0, 1.0) }
	pub const fn light_magenta() -> Color { Color::rgb(1.0, 0.5, 1.0) }

	pub fn to_byte_tuple(&self) -> (u8, u8, u8, u8) {
		let Color{r,g,b,a} = *self;
		((r*255.0) as u8, (g*255.0) as u8, (b*255.0) as u8, (a*255.0) as u8)
	}

	pub fn to_tuple(&self) -> (f32, f32, f32, f32) {
		let Color{r,g,b,a} = *self;
		(r,g,b,a)
	}

	pub fn to_byte_array(self) -> [u8; 4] { self.into() }
	pub fn to_array(self) -> [f32; 4] { self.into() }

	pub fn to_vec3(&self) -> Vec3 { Vec3::new(self.r, self.g, self.b) }
	pub fn to_vec4(&self) -> Vec4 { Vec4::new(self.r, self.g, self.b, self.a) }

	pub fn with_alpha(&self, a: f32) -> Color {
		Color { a, ..*self }
	}

	pub fn pow(self, exp: f32) -> Color {
		Color::rgba(
			self.r.powf(exp),
			self.g.powf(exp),
			self.b.powf(exp),
			self.a,
		)
	}

	pub fn to_srgb(&self) -> Color {
		Color::rgba(
			linear_channel_to_srgb(self.r),
			linear_channel_to_srgb(self.g),
			linear_channel_to_srgb(self.b),
			self.a,
		)
	}

	pub fn to_linear(&self) -> Color {
		Color::rgba(
			srgb_channel_to_linear(self.r),
			srgb_channel_to_linear(self.g),
			srgb_channel_to_linear(self.b),
			self.a,
		)
	}
}

impl From<(u8,u8,u8)> for Color {
	fn from(o: (u8,u8,u8)) -> Color { Color::rgb8(o.0, o.1, o.2) }
}
impl From<(u8,u8,u8,u8)> for Color {
	fn from(o: (u8,u8,u8,u8)) -> Color { Color::rgba8(o.0, o.1, o.2, o.3) }
}
impl From<(f32,f32,f32)> for Color {
	fn from(o: (f32,f32,f32)) -> Color { Color::rgb(o.0, o.1, o.2) }
}
impl From<(f32,f32,f32,f32)> for Color {
	fn from(o: (f32,f32,f32,f32)) -> Color { Color::rgba(o.0, o.1, o.2, o.3) }
}

impl From<[u8; 3]> for Color {
	fn from([r, g, b]: [u8; 3]) -> Color { Color::rgb8(r, g, b) }
}
impl From<[u8; 4]> for Color {
	fn from([r, g, b, a]: [u8; 4]) -> Color { Color::rgba8(r, g, b, a) }
}

impl From<[f32; 3]> for Color {
	fn from([r, g, b]: [f32; 3]) -> Color { Color::rgb(r, g, b) }
}
impl From<[f32; 4]> for Color {
	fn from([r, g, b, a]: [f32; 4]) -> Color { Color::rgba(r, g, b, a) }
}


impl From<Vec3> for Color {
	fn from(o: Vec3) -> Color { Color::rgb(o.x, o.y, o.z) }
}
impl From<Vec4> for Color {
	fn from(o: Vec4) -> Color { Color::rgba(o.x, o.y, o.z, o.w) }
}

impl From<Color> for Vec3 {
	fn from(o: Color) -> Vec3 { o.to_vec3() }
}
impl From<Color> for Vec4 {
	fn from(o: Color) -> Vec4 { o.to_vec4() }
}

impl From<Color> for [f32; 3] {
	fn from(Color{r, g, b, ..}: Color) -> [f32; 3] { [r, g, b] }
}
impl From<Color> for [f32; 4] {
	fn from(Color{r, g, b, a}: Color) -> [f32; 4] { [r, g, b, a] }
}

impl From<Color> for (f32,f32,f32) {
	fn from(Color{r, g, b, ..}: Color) -> (f32,f32,f32) { (r, g, b) }
}
impl From<Color> for (f32,f32,f32,f32) {
	fn from(Color{r, g, b, a}: Color) -> (f32,f32,f32,f32) { (r, g, b, a) }
}

impl From<Color> for [u8; 3] {
	fn from(Color{r, g, b, ..}: Color) -> [u8; 3] {
		[r, g, b].map(|v| (v.clamp(0.0, 1.0)*255.0) as u8)
	}
}
impl From<Color> for [u8; 4] {
	fn from(Color{r, g, b, a}: Color) -> [u8; 4] {
		[r, g, b, a].map(|v| (v.clamp(0.0, 1.0)*255.0) as u8)
	}
}
impl From<Color> for (u8,u8,u8) {
	fn from(o: Color) -> (u8,u8,u8) {
		let [r, g, b]: [u8; 3] = o.into();
		(r, g, b)
	}
}
impl From<Color> for (u8,u8,u8,u8) {
	fn from(o: Color) -> (u8,u8,u8,u8) {
		let [r, g, b, a]: [u8; 4] = o.into();
		(r, g, b, a)
	}
}




impl Lerp<Color> for f32 {
	fn lerp(self, start: Color, end: Color) -> Color {
		Color {
			r: self.lerp(start.r, end.r),
			g: self.lerp(start.g, end.g),
			b: self.lerp(start.b, end.b),
			a: self.lerp(start.a, end.a),
		}
	}
}


impl Default for Color {
	fn default() -> Color {
		Color::black()
	}
}



fn srgb_channel_to_linear(value: f32) -> f32 {
	// https://en.wikipedia.org/wiki/SRGB#From_sRGB_to_CIE_XYZ
	if value <= 0.04045 {
		value / 12.92
	} else {
		((value + 0.055) / 1.055).powf(2.4)
	}
}

fn linear_channel_to_srgb(value: f32) -> f32 {
	// https://en.wikipedia.org/wiki/SRGB#From_CIE_XYZ_to_sRGB
	if value <= 0.0031308 {
		value * 12.92
	} else {
		value.powf(1.0/2.4) * 1.055 - 0.055
	}
}




#[test]
fn test_srgb_conversion() {
	let srgb_value = 0.5;
	let linear_value = srgb_channel_to_linear(srgb_value);
	let restored_value = linear_channel_to_srgb(linear_value);

	assert!((srgb_value - restored_value).abs() < 0.00001);
}
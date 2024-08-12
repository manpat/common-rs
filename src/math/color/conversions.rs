use super::*;

// To Color
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

// From Color
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


impl AsRef<[f32; 3]> for Color {
	fn as_ref(&self) -> &[f32; 3] {
		/// SAFETY: #[repr(C)] guarantees that Color is laid out the same way as [f32; 4],
		/// which contains [f32; 3] as an initial sequence.
		unsafe { std::mem::transmute(self) }
	}
}

impl AsMut<[f32; 3]> for Color {
	fn as_mut(&mut self) -> &mut [f32; 3] {
		/// SAFETY: #[repr(C)] guarantees that Color is laid out the same way as [f32; 4],
		/// which contains [f32; 3] as an initial sequence.
		unsafe { std::mem::transmute(self) }
	}
}


impl AsRef<[f32; 4]> for Color {
	fn as_ref(&self) -> &[f32; 4] {
		/// SAFETY: #[repr(C)] guarantees that Color is laid out the same way as [f32; 4]
		unsafe { std::mem::transmute(self) }
	}
}

impl AsMut<[f32; 4]> for Color {
	fn as_mut(&mut self) -> &mut [f32; 4] {
		/// SAFETY: #[repr(C)] guarantees that Color is laid out the same way as [f32; 4]
		unsafe { std::mem::transmute(self) }
	}
}
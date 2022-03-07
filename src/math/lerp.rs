pub trait Lerp<Bound> {
	fn lerp(self, start: Bound, end: Bound) -> Bound;
}


impl Lerp<f32> for f32 {
	fn lerp(self, start: f32, end: f32) -> f32 {
		start + (end - start) * self
	}
}



// http://robertpenner.com/easing/
// http://easings.net/

pub trait EaseFloatExt {
	fn ease_linear(self) -> Self;

	fn ease_quad_in(self) -> Self;
	fn ease_quad_out(self) -> Self;
	fn ease_quad_inout(self) -> Self;

	fn ease_exp_in(self) -> Self;
	fn ease_exp_out(self) -> Self;
	fn ease_exp_inout(self) -> Self;

	fn ease_back_in(self) -> Self;
	fn ease_back_out(self) -> Self;
	fn ease_back_inout(self) -> Self;

	fn ease_bounce_in(self) -> Self;
	fn ease_bounce_out(self) -> Self;
	fn ease_bounce_inout(self) -> Self;
}


impl EaseFloatExt for f32 {
	fn ease_linear(self) -> f32 { self }

	fn ease_quad_in(self) -> f32 {
		self.clamp(0.0, 1.0).powi(2)
	}

	fn ease_quad_out(self) -> f32 {
		let t = self.clamp(0.0, 1.0);
		-t * (t - 2.0)
	}

	fn ease_quad_inout(self) -> f32 {
		let t = self.clamp(0.0, 1.0) * 2.0;
		if t < 1.0 {
			t.powi(2) / 2.0
		} else {
			-((t - 1.0) * (t - 3.0) - 1.0) / 2.0
		}
	}

	fn ease_exp_in(self) -> f32 {
		let t = self.clamp(0.0, 1.0);
		2.0f32.powf(10.0 * (t - 1.0))
	}

	fn ease_exp_out(self) -> f32 {
		let t = self.clamp(0.0, 1.0);
		1.0 - 2.0f32.powf(-10.0 * t)
	}

	fn ease_exp_inout(self) -> f32 {
		let t = self.clamp(0.0, 1.0) * 2.0;
		if t < 1.0 {
			2.0f32.powf(10.0 * (t - 1.0)) / 2.0
		} else {
			1.0 - 2.0f32.powf(-10.0 * (t - 1.0)) / 2.0
		}
	}


	fn ease_back_in(self) -> f32 {
		let s = 1.70158;
		let t = self.clamp(0.0, 1.0);
		t*t*((s+1.0)*t - s)
	}

	fn ease_back_out(self) -> f32 {
		let s = 1.70158;
		let t = self.clamp(0.0, 1.0) - 1.0;
		t*t*((s+1.0)*t + s) + 1.0
	}

	fn ease_back_inout(self) -> f32 {
		let t = self.clamp(0.0, 1.0) * 2.0;

		if t < 1.0 {
			t.ease_back_in() / 2.0
		} else {
			(t - 1.0).ease_back_out() / 2.0 + 0.5
		}
	}


	fn ease_bounce_in(self) -> f32 {
		1.0 - (1.0 - self).ease_bounce_out()
	}

	fn ease_bounce_out(self) -> f32 {
		let t = self.clamp(0.0, 1.0);

		let fact = 7.5625;

		if t < 1.0/2.75 {
			fact*t*t
		} else if t < 2.0/2.75 {
			let t = t - 1.5/2.75;
			fact*t*t + 0.75
		} else if t < 2.5/2.75 {
			let t = t - 2.25/2.75;
			fact*t*t + 0.9375
		} else {
			let t = t - 2.625/2.75;
			fact*t*t + 0.984375
		}
	}

	fn ease_bounce_inout(self) -> f32 {
		let t = self.clamp(0.0, 1.0) * 2.0;

		if t < 1.0 {
			t.ease_bounce_in() / 2.0
		} else {
			(1.0 + (t - 1.0).ease_bounce_out()) / 2.0
		}
	}
}


pub struct Defer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Defer<F> {
	pub fn new(f: F) -> Self {
		Defer(Some(f))
	}

	pub fn reset(&mut self) {
		self.0 = None;
	}
}

impl<F> Drop for Defer<F>
	where F: FnOnce()
{
	fn drop(&mut self) {
		(self.0.take().unwrap())();
	}
}


pub fn defer<F>(f: F) -> Defer<F>
	where F: FnOnce()
{
	Defer::new(f)
}
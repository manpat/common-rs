

#[macro_export]
macro_rules! internal_vec_map {
	(@apply ($v:expr, $el:tt), @[$($body:tt)*] element $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[$($body)* $v.$el] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*] $name:ident.element $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[$($body)* $name.$el] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*] ( $($subexpr:tt)+ ) $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[
			$($body)*
			( internal_vec_map!(@apply ($v, $el), @[] $($subexpr)+) )
		] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*] { $($subexpr:tt)+ } $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[
			$($body)*
			{ internal_vec_map!(@apply ($v, $el), @[] $($subexpr)+) }
		] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*] $next:tt $($tail:tt)* ) => {
		internal_vec_map!(@apply ($v, $el), @[$($body)* $next] $($tail)*)
	};

	(@apply ($v:expr, $el:tt), @[$($body:tt)*]) => { $($body)* };

	(Vec2 $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec2 {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
		}
	}};

	(Vec2i $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec2i {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
		}
	}};

	(Vec3i $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec3i {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
			z: internal_vec_map!(@apply (v, z), @[] $($func)+),
		}
	}};

	(Vec3 $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec3 {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
			z: internal_vec_map!(@apply (v, z), @[] $($func)+),
		}
	}};

	(Vec4 $v:expr, $($func:tt)+) => {{
		let v = $v;
		Vec4 {
			x: internal_vec_map!(@apply (v, x), @[] $($func)+),
			y: internal_vec_map!(@apply (v, y), @[] $($func)+),
			z: internal_vec_map!(@apply (v, z), @[] $($func)+),
			w: internal_vec_map!(@apply (v, w), @[] $($func)+),
		}
	}};
}

#[macro_export]
macro_rules! vec2_map { ($($tt:tt)+) => { internal_vec_map!(Vec2 $($tt)+) } }

#[macro_export]
macro_rules! vec2i_map { ($($tt:tt)+) => { internal_vec_map!(Vec2i $($tt)+) } }

#[macro_export]
macro_rules! vec3i_map { ($($tt:tt)+) => { internal_vec_map!(Vec3i $($tt)+) } }

#[macro_export]
macro_rules! vec3_map { ($($tt:tt)+) => { internal_vec_map!(Vec3 $($tt)+) } }

#[macro_export]
macro_rules! vec4_map { ($($tt:tt)+) => { internal_vec_map!(Vec4 $($tt)+) } }
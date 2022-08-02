#[macro_export]
macro_rules! include_png {
	($path:literal) => {
		format_args!(
			"data:image/png;base64,{}",
			base64::encode(include_bytes!($path))
		)
	};
}
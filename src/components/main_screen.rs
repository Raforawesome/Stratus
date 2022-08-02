#[macro_export]
macro_rules! main_screen_component {
	() => {
		rsx!(
			style { [include_str!("./css/main_screen.css")] }
			img { 
				src: include_png!("./assets/images/logo-full.png"),
				height: "100px",
				class: "logo"
			}
		)
	}
}
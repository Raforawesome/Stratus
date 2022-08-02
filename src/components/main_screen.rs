#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{ AppProps, include_png };

pub fn MainScreen(cx: Scope<AppProps>) -> Element {
	cx.render(rsx!(
		style { [include_str!("../css/main_screen.css")] }
		img { 
			src: include_png!("../assets/images/logo-full.png"),
			height: "100px",
			class: "logo"
		}
	))
}
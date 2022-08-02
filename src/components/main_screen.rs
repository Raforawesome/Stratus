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
		input { style: "margin-top:12vh;", placeholder: "Username" }
		input { "type": "password", style: "margin-top:3vh;", placeholder: "Password" }
		button {
			onclick: move |_| {
				println!("Log in button clicked")
			},
			"Log In"
		}
	))
}
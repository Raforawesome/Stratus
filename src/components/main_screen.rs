#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::{ include_png, functions::login };

pub fn MainScreen(cx: Scope) -> Element {
	cx.render(rsx!(
		style { [include_str!("../css/main_screen.css")] }
		img { 
			src: include_png!("../assets/images/logo-full.png"),
			height: "100px",
			class: "logo"
		}
		// input { style: "margin-top:12vh;", placeholder: "Username", id: "username" }
		input { "type": "password", style: "margin-top:15vh;", placeholder: "Password", id: "password" }
		button {
			onclick: move |_| {
				println!("Log in button clicked");
				if login::try_login("e", "e") {
				    println!("Successful login");
					let router = use_router(&cx);
					router.replace_route("/dash", None, None);
				} else {
				    println!("Unsuccessful login");
				    // Include error shake & message here
				}
			},
			"Unlock"
		}
	))
}
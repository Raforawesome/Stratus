#![allow(non_snake_case)]
use crate::{functions::login, include_png};
use dioxus::prelude::*;

pub fn RegisterScreen(cx: Scope) -> Element {
    let password_input = use_state(&cx, || "".to_string());
	let router = use_router(&cx);

	if crate::functions::login::is_registered() {
		router.replace_route("/login", None, None);
	}

    cx.render(rsx!(
        style { [include_str!("../css/register_screen.css")] }
        img {
            src: include_png!("../assets/images/logo-full.png"),
            height: "100px",
            class: "logo"
        }
        input { style: "margin-top:12vh;", placeholder: "Name (only used for greetings)", id: "username" }
        input {
            oninput: |event| {
                password_input.set(event.value.clone());
            },
            "type": "password",
            style: "margin-top:3vh;",
            placeholder: "Password",
            id: "password"
        }
        button {
            onclick: move |_| {
                if login::try_login(password_input.get()) {
                    router.replace_route("/login", None, None);
                } else {
                    // Include error shake & message here
                }
            },
            "Setup"
        }
    ))
}

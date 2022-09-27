#![allow(non_snake_case)]
use crate::{functions::login, include_png};
use dioxus::prelude::*;

pub fn LoginScreen(cx: Scope) -> Element {
    let password_input = use_state(&cx, || "".to_string());
    let show_error = use_state(&cx, || true);
    let to_show_error: String = (*show_error.get()).to_string();

    cx.render(rsx!(
        style { [include_str!("../css/login_screen.css")] }
        img {
            src: include_png!("../assets/images/logo-full.png"),
            height: "100px",
            class: "logo"
        }
        // input { style: "margin-top:12vh;", placeholder: "Username", id: "username" }
        input {
            oninput: |event| {
                password_input.set(event.value.clone());
            },
            "type": "password",
            style: "margin-top:15vh;",
            placeholder: "Password",
            id: "password"
        }
        button {
            onclick: move |_| {
                if login::try_login(password_input.get()) {
                    // println!("Successful login");
                    let router = use_router(&cx);
                    router.replace_route("/dash", None, None);
                    show_error.set(true);
                } else {
                    // println!("Unsuccessful login");
                    show_error.set(false);
                    // Include error shake & message here
                }
            },
            "Unlock"
        }
        p {
            class: "error-text",
            hidden: "{to_show_error}",
            "Incorrect password!"
        }
    ))
}

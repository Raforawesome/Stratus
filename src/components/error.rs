#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::include_png;

pub fn ErrorScreen(cx: Scope) -> Element {
    cx.render(rsx!(
        style { [include_str!("../css/error_screen.css")] }
        img { src: include_png!("../assets/images/logo-full.png") }
        h1 { "Error! Something is broken" }
        h4 { "You aren't supposed to see this screen..." }
    ))
}
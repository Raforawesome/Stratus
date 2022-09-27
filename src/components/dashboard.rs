#![allow(non_snake_case)]
use crate::functions::login;
use dioxus::prelude::*;
// use crate::include_png;

pub fn Dashboard(cx: Scope) -> Element {
    let name: String = login::get_name();
    cx.render(rsx!(
        style { [include_str!("../css/dashboard.css")] }
        h1 { class: "title", "Welcome back, {name}!" }
    ))
}

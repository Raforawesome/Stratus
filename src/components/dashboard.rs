#![allow(non_snake_case)]
use dioxus::prelude::*;
// use crate::include_png;

pub fn Dashboard(cx: Scope) -> Element {
    cx.render(rsx!(
        h1 { "Dashboard" }
    ))
}

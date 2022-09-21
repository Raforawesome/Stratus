#![feature(once_cell)]
#![allow(unused)]
mod macros;
mod components;
mod functions;
use dioxus::prelude::*;
use components::{
	MainScreen,
	Dashboard,
	ErrorScreen
};


fn main() {
    dioxus::desktop::launch_cfg(
        app,
        |c| {
            c.with_disable_context_menu(true);
            c.with_window(|w| {
                w.with_resizable(false)
                    .with_title("Stratus")
            })
        }
    );
}

fn app(cx: Scope) -> Element {
    // Actual rendering
    cx.render(rsx!(
        style { [include_str!("./css/global.css")] }
        Router {
            Route { to: "/", MainScreen {} }
            Route { to: "/dash", Dashboard {} }
            Route { to: "", ErrorScreen {} }
        }
    ))
}
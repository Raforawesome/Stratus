#![feature(once_cell)]
mod macros;
mod components;
mod functions;
use dioxus::prelude::*;
use std::sync::{ LazyLock, Mutex };
use components::{
	MainScreen,
	Dashboard
};


#[derive(PartialEq, Eq, Props)]
pub struct AppProps {
    screen: &'static str
}

pub static SCREEN: LazyLock<Mutex<&'static str>> = LazyLock::new(|| {
    Mutex::new("main")
});


fn main() {
    dioxus::desktop::launch_with_props(
        app,
        AppProps { screen: "main" },
        |c| {
            c.with_disable_context_menu(true);
            c.with_window(|w| {
                w.with_resizable(false)
                    .with_title("Stratus")
            })
        }
    );
}

fn app(cx: Scope<AppProps>) -> Element {
    // Actual rendering
    cx.render(rsx!(
        style { [include_str!("./css/global.css")] }
        Router {
            Route { to: "/home", MainScreen {} }
            Route { to: "/dash" Dashboard {} }
        }
    ))
}
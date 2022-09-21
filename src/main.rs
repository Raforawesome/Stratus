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


#[derive(PartialEq, Eq, Props)]
pub struct AppProps {
    screen: &'static str
}


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
            Route { to: "/", MainScreen {} }
            Route { to: "/dash", Dashboard {} }
            Route { to: "", ErrorScreen {} }
        }
    ))
}
mod macros;
mod components;
use dioxus::{ prelude::*, desktop::use_window };
use components::{
	MainScreen
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
            c.with_disable_context_menu(true)
        }
    );
}

fn app(cx: Scope<AppProps>) -> Element {
	// Window setup
	let window_handle = use_window(&cx);
	window_handle.set_resizable(false);

	// Actual rendering
    cx.render(rsx!(
		style { [include_str!("./css/global.css")] }
		match cx.props.screen {
			"main" => cx.render(rsx!(
				MainScreen { screen: cx.props.screen }
			)),
			_ => {
				eprintln!("Invalid screen!");
				std::process::exit(1);
			}
		}
    ))
}
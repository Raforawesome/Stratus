use dioxus::prelude::*;

struct AppProps {
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
    cx.render(rsx!(
        h1 { "Hello, world!" }
    ))
}
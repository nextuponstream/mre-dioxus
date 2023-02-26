use dioxus::prelude::*;

fn main() {
    hot_reload_init!();
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { "Hello, world!" }
    ))
}

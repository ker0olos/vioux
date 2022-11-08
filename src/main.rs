#[cfg(feature = "desktop")]
use dioxus::prelude::*;

#[tokio::main]
#[cfg(feature = "desktop")]
async fn main() {
    vioux::server::spawn();

    dioxus::desktop::launch_cfg(app, |c| {
        c.with_window(|w| {
            //
            w.with_title("vioux")
        })
    });
}

#[cfg(feature = "desktop")]
fn app(cx: Scope) -> Element {
    // let window = dioxus::desktop::use_window(&cx);

    cx.render(rsx! (
        div { "Hello, world!" }
    ))
}

#[cfg(not(feature = "desktop"))]
fn main() {
    panic!()
}

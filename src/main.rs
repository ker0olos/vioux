#[cfg(feature = "desktop")]
use dioxus::prelude::*;

#[cfg(feature = "desktop")]
fn main() {
    dioxus::desktop::launch(|cx| {
        cx.render(rsx! (
            div { "Hello, world!" }
        ))
    });
}

#[cfg(not(feature = "desktop"))]
fn main() {
    panic!()
}

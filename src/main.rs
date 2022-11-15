// #[cfg(feature = "desktop")]
// use dioxus::{desktop::tao, prelude::*};

#[tokio::main]
#[cfg(feature = "desktop")]
async fn main() -> anyhow::Result<()> {
    vioux::grpc::server::spawn()?;

    // dioxus::desktop::launch_cfg(app, |c| {
    //     c.with_window(|w| {
    //         w.with_title("vioux")
    //             .with_min_inner_size(tao::dpi::LogicalSize::new(400.0, 200.0))
    //     })
    // });

    loop {
        //
    }

    Ok(())
}

// #[cfg(feature = "desktop")]
// fn app(cx: Scope) -> Element {
//     // let window = dioxus::desktop::use_window(&cx);

//     cx.render(rsx! (
//         div { "Hello, world!" }
//     ))
// }

#[cfg(not(feature = "desktop"))]
fn main() {
    unreachable!("Running on an unsupported platform!")
}

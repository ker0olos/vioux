// #[cfg(feature = "desktop")]
// use dioxus::{desktop::tao, prelude::*};

use vioux::{ViouxServer, ViouxService};

#[tokio::main]
#[cfg(feature = "desktop")]
async fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:50051".parse()?;

    let vioux_service_impl = ViouxService::default();

    tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(ViouxServer::new(vioux_service_impl))
            .serve(addr)
            .await
    });

    // dioxus::desktop::launch_cfg(app, |c| {
    //     c.with_window(|w| {
    //         w.with_title("vioux")
    //             .with_min_inner_size(tao::dpi::LogicalSize::new(400.0, 200.0))
    //     })
    // });

    loop {}

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

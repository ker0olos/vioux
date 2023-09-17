<div align="center">

# vioux

A proof-of-concept for a scripting-based video editor

</div>

## Proof of concept

vioux stands for "video input output user experience".

Proof-of-concept in this context means that I'm still in the process of trying to proof to myself that this is actually useful and whether or not, and if it's worth fully-fledging.

The idea is to create a scripting-based video editor with bare-minimum features included in the box, anything else is added through community plugins and user-written code.

## Why

I been using kdenlive for a while now, but it's UI is unfriendly (like everything else from KDE/QT), and I can't figure out how to add custom effects. I wished there was an way to drag code into the timeline, and that's how this idea was born.

## Implementation

The app is built with rust for speed, cross-platform, multithreading, etc.

<!-- The UI uses [dioxus](https://github.com/DioxusLabs/dioxus) which is a react-like GUI library that uses webview, because css is the best human innovation since fire and [iced](https://github.com/iced-rs/iced) is not yet production-ready. -->

Scripts [plugins] connect with vioux through a local gRPC server.

Any script (in any language) can technically connect to the [gRPC] server. But other than python, scripts will have to be valid executables, meaning, vioux won't compile your code (e.g. rust) before rendering, you will have to manually compile it yourself beforehand, or use a valid shebang, in case of languages like bash, and javascript.

> **Note**
> Scripts are called (executed) during rendering and while seeking around in the project timeline.

With that in mind, python will always be easiest option with a lot of support out of the box.

You will have access to a library full of helper utilities. Including functions that automatically connects the the server, so users can avoid all direct contact with the grpc-related apis. The library also comes with many functions that do popular image and audio processing tasks like rotation, translation, and gaining.

## Dev

```bash
python3 -m venv .venv
. .venv/bin/activate
pip install maturin
maturin develop
cargo run
```

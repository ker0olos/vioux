<h1 align="center">vioux</h1>

vioux stands for "video input output user experience"

The idea is to create a scripting-based video editor with bare-minimum features
included in the box, anything else is added through community plugins and
user-written code.

> [!IMPORTANT]\
> Vioux is in early development and you can't download it or use it!

## Why

I been using kdenlive for a while now, but it's UI is unfriendly (like
everything else from KDE/QT), and I can't figure out how to add custom effects.
I wished there was an way to drag code into the timeline, and that's how this
idea was born.

## Implementation

The app is built with rust for speed, cross-platform, multithreading, etc.

Scripts [plugins] connect with vioux through a local gRPC server.

Any script (in any language) can technically connect to the [gRPC] server. But
other than python, scripts will have to be valid executables, meaning, vioux
won't compile your code (e.g. rust) before rendering, you will have to manually
compile it yourself beforehand, or use a valid shebang, in case of languages
like bash, and javascript.

> [!Note]\
> Scripts are called (executed) during rendering and while seeking around in the
> project timeline.

With that in mind, that our official library is only for python, it
automatically connects the the server, so users can avoid direct contact with
the grpc-related APIs.

## Development

```sh
python3 -m venv .venv
. .venv/bin/activate
pip install maturin
cd vioux_lib && maturin develop
cargo run
```

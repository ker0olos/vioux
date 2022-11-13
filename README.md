<div align="center">

# vioux

A proof-of-concept for a scripting-based video editor
</div>

# Proof of concept

vioux stands for "video input output user experience" (the name is a placeholder).

Proof-of-concept in this context means that I'm trying to proof to myself that this is actually useful and whether or not it's worth fully-fledging.

The idea is to create a scripting-based video editor with bare-minimum features included in the box, anything else is added through community plugins and user-written code.

## Why

I been using kdenlive for a while now, but it's UI is unfriendly (like everything else from KDE/QT), and I can't figure out how to add custom effects. I wished there was an way to drag code into the timeline, and that's when this project started. I don't want my creativity to die because the app i am using is limited.

## Implementation

The app is built with rust for speed, cross-platform, multithreading, etc.

The UI uses [dioxus](https://github.com/DioxusLabs/dioxus) which is a react-like GUI library because css is the best human innovation since fire and [iced](https://github.com/iced-rs/iced) is still not production-ready.

Scripts [plugins] connect with vioux through a local gRPC server (with reflection).

> **Note**
> Plugins are called [executed] during rendering and while seeking around in the project timeline.

Any script (in any language) can technically connect to the [gRPC] server. But other than python, scripts will have to be valid executables, for example, vioux won't compile your rust code before rendering, you will have to manually compile it yourself. vioux will only call it when it's needed.

With that in mind, python will be easiest option with a lot of support out of the box. You will have access to a library full of helper utilities. Including functions to easily connect the the server and avoid any direct contact with gRPC-related apis. Plus functions that do popular image and audio processing tasks like rotation, translation, and gaining.

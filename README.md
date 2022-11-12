# vioux

A proof-of-concept scripting-based video editor

vioux stands for "video input output user experience" (the name is a placeholder). proof-of-concept means that i'm trying to proof to myself that this is actually useful and whether or not it's worth fully-fledging.

The idea of the project is to create scripting-based video editor with bare-minimum features included, anything else is added through community extensions and user-written code.

I been using kdenlive for a while now, but it's UI is unfriendly (like everything else from KDE/QT), and I can't figure out how to add custom effects. I wished there was an way to drag code into the timeline, and that's when this project started. I don't want my creativity to die because the app i am using is limited.

The app is built with rust for speed, cross-platform, multithreading, etc. the UI uses <https://dioxuslabs.com/guide> because css is the innovation since fire.

Any script in any language can technically connect to the vioux's gRPC server. But other than python, scripts will have to be valid executables, for example, vioux won't compile your rust code before rendering, you will have to manually compile it yourself. vioux will only call it when it's needed.

With that in mind, python will be easier with a lot of support out of the box. You will get accese to a library full of helper utilities. Including functions to easily connect the the gRPC server and avoid any direct contact with gRPC-related apis. Plus functions that do popular image and audio processing tasks like rotation, translation, and gaining.

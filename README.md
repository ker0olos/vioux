# vioux
A proof-of-concept scripting-based video editor

vioux stands for video input output user experience (the name is a placeholder).
proof-of-concept means that i'm trying to proof to myself that this is actually useful and whether or not it's worth fully-fledging

the idea of the project is to create scripting-based video editor with bare-minimum features included, everything else is added through community addons and/or user-written code

the main focus points of this project isn't features but the ui/ux and the scripting library

i been using kdenlive for a while now but it's ui is too unfriendly (like everything else from KDE/QT) and I can't figure out how to add custom effects, i wished there was an way to drag code into the timeline and this is how this idea started, i don't want my creativity to die just because the app i am using is limited.

the app is built with rust for speed, cross-platform, multithreading, and because rust is generally awesome.
the ui uses https://dioxuslabs.com/guide because css is the best thing to ever happen to humans.

the scripting library supports both rust and python for accessibility; python is easier-to-use and much better for writing small scripts than rust.
the python bindings are generated using https://pyo3.rs/v0.17.3/getting_started.html

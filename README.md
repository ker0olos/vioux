# vioux
A proof-of-concept scripting-based video editor

vioux stands for video input output user experience
the name is a placeholder

the idea of the project is to create scripting-based video editor
with bare-minimum features included and everything else is added through community addons and/or user scripts
so the main focus points of this project isn't features but the ui/ux and the scripting library

the app will be build with rust for speed and cross-platform
idealy using either https://github.com/iced-rs/iced or https://github.com/DioxusLabs/dioxus

the scripting library should support both rust and python for accessibility
since python is easier-to-use and much better for writing small scripts than rust
for the python bindings pyo3 is used
https://pyo3.rs/v0.17.3/getting_started.html

# How layouting works

There are many different way you can layout your ui.
Egui provides the Layout struct, Grid, horizontals and verticals, columns and many more in third party crates like
the flex layout in egui_extra, egui_taffy or egui_flex.

At the most fundamental level you will always eventually add widgets to a ui.
Those widgets are layed out using the layout flow described with the `Layout` of the ui.


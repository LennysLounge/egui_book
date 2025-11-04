# Main direction again

The main direction also has an alignment and justification setting with the `main_align` and `main_justify` fields.
In principle these work exactly the same as the cross direction, however the effect can be a little confusing.

## Justification

When the layout is justified in the main direction, any widget added to the ui will expand to take up all the available space in the 
main direction.

This has the unfortunate/intended side effect that only one widget will be visible. Since the first widget takes up all the available space,
the second widget is forced to be placed outside of the ui's bounds and very likely wont be visible.

Enabling *Draw response rectangles* eluminates this fact and shows that the label consumes all available space leaving no space for the other widgets.
In a top down layout you might be able to see the very top of the "button 2" at the bottom of the bottom of the ui.

{{sample: layout_main_justify}}

## Alignment

Similar to the alignment of the corss direction, the alignment of the main direction determines where a widget is placed within its available space.
In layouts where `main_justify` is false the space that is available to the widget is only ever as large as the widget itself. Therefore the `main_align`
is very unlikely to have a visible effect.

The effects become visible if the `main_justify` is set to true. In this case, the space available to the widget is the remainging space of the ui and the
alignment becomes visible.

{{sample: layout_main_align}}



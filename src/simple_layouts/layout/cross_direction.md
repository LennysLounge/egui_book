# The cross direction

Perpendicular to the main direction is the cross direction.
If the main direction is vertical then the cross direction is horizontal and vice versa.

## Alignment
The `cross_align` field of a layout determines where a widget is placed on the cross axis.

* **Horizontal main axis** therefore means a **vertical cross axis**
    * `Min` widgets are placed at the top of the ui
    * `Center` widgets are placed centered in the ui
    * `Max` widgets are placed at the bottom of the ui
* **Vertical main axis** therefore means a **horizontal cross axis**
    * `Min` widgets are placed at the left of the ui
    * `Center` widgets are placed centered in the ui
    * `Max` widgets are placed at the right of the ui

{{sample: layout_cross_align}}

## Justification
If the `cross_justify` field is set to true, then any widget added to the ui will be expanded alongs the cross axis
to take up all the available space in that axis.

For a **horizontal main axis** this means a **vertical cross axis** and widgets will expand to the full height of the ui.  
For a **vertical main axis** this means a **horizontal cross axis** and widgets will expand to the full width of the ui.  

{{sample: layout_cross_justify}}

Some things to note for this demo:

At first glance the label and the checkbox dont appear to be affected by the justify option.
If you enable the *Draw response rectangles* option you can see that the response returned by those widgets is in fact stretched to the available size.
This can have an impact on the click behavior of widgets. For example, in a justified layout the checkbox widget will react to clicks and hovers on the entire area of its
response rectangle.

The text alignment within a button changes depending on the cross alignment. This is true regardless of the justify option but it only becomes visible once the frame of the button is larger
than the text itself.
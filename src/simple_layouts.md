# Simple layouts

I consider simple layouts to be layouts that only use the layouting tools provided by egui itself.
These include the [`Layout`](https://docs.rs/egui/latest/egui/struct.Layout.html) struct, [`Grid`](https://docs.rs/egui/latest/egui/struct.Grid.html)
and the various convenience methods on [`Ui`](https://docs.rs/egui/latest/egui/struct.Ui.html) like [`horizontal`](https://docs.rs/egui/latest/egui/struct.Ui.html#method.horizontal)
and [`vertical`](https://docs.rs/egui/latest/egui/struct.Ui.html#method.vertical).

# How the layout flow works in egui.

## The cursor
A ui contains a special rectangle called the cursor.
The cursor is the area inside of a ui where widgets can be placed.
As more and more widgets are added to the ui, the cursor will shrink to stop widgets from overlapping.
You can query the current cursor using the [`Ui::cursor`](https://docs.rs/egui/0.33.0/egui/struct.Ui.html#method.cursor) method.

In this example i have colored the colored the rectangle of the cursor in red.
Add you add more widgets to the ui the cursor moves down.

{{sample: visualize_cursor}}

You can notice that the cursor touches the bottom of the viewport.
This is because, in this case, the cursor has infinite height.
Printing the cursor rect to a label shows this.

{{sample: visualize_cursor_size}}

Visualizing the cursor like this can be a great way to understand why the layout flow works the way it does.

## The layout flow


When adding a widget to a ui, first the layout flow will choose a position inside the cursor where the widget will be placed, then

When adding a widget to a ui, the layout flow will choose an 
approriate position inside the cursor where the widget will be placed .

When adding a widget to a ui it is the layout flow that decides where the widget is placed.
The layout flow is controlled by the [`Layout`](https://docs.rs/egui/latest/egui/struct.Layout.html) of the current ui.



It is not possible to modify the layout of the current ui but you can create a new sub-ui with a specific layout by using [`Ui::with_layout`](https://docs.rs/egui/0.33.0/egui/struct.Ui.html#method.with_layout).

A ui contains a special rectangle called the cursor. The layout flow will choose an appropriate position inside the cursor to place a widget.

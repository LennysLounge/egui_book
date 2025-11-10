# Maximum size

Ui's have a maximum size.
As the name suggests, this is the maximum size that widgets in the ui are allowed to take up.
You can very easily query the maximum size by calling `Ui::ui.max_rect()`.
If you want to know who much space is left in the ui you can do that by calling `Ui::available_width()`, `Ui::available_heigh()` and `ui::available_size()`.

The maximum size constraint is not enforced at all.
There is nothing that prevents you from growing the ui beyond the maximum size.
The maximum size will simply get bigger to encompass all widgets just like the minimum size does.
What exactly happens when you grow beyond the maximum size depends on the parent container.

Here are a few examples and issues you might run into relating to the maximum size of a ui.

## Maximum size in a window
The ui of a window has a maximum size too.
In the sample i have colored the maximum rect in red to make it visible.
Like discussed in the previous section, the windows size is derived from the minimum size of the ui.
For that reason the window is just large enough to show the label and the red rectangle overflows the window.

When you resize the window what you are doing is you are changing the maximum size of the ui.
{{sample: ui_max_size_window}}

## Resizing windows
In the sample above you can see that you can drag the resize handle but nothing happens.
This is because the window always draws its frame around the ui's minimum size.
Since that window only contains the label, the window is always going to be the size of the label.

If you want your window to respond to the resizing you must make sure the minimum size of the window is always at least as large as the maximum size.
There are multiple ways you can achieve this.
The easiest way is to just tell the ui to take up all the available space using `Ui::take_available_space()`.

The other options involve using a widget that expands to the available width/height itself or changing the layout to take advantage of the available width/height
{{sample: ui_max_size_window_resize}}

The button in the "h centered" window is centered on the horizontal axis and therefore takes up all the available width.
This makes the window resizeable on the horizontal axis.
The button in the "v centered" window is vertically centered and therefore takes up all the available height.
This makes the window resizeable on the vertical axis.


## Infinit growth
A window takes it maximum size from its minimum size on the previous frame (this is true for many other containers and widgets but a window is a good example).
If you happen to take more space than the maximum size then the window will simply grow on the next frame to account for the bigger ui.
If you do this every frame then the window will every frame and quickly grow out of control.

The easiest way to provoke this to take all the available size, and then a little more.
Press the reset button in the sample below to reset the size of the window and watch it grow.
{{sample: ui_max_size_infinit_growth}}


## Maximum size in a central panel
A central panel takes up the entire viewport (minus left, right, top and bottom panels).
Since the viewport cannot get bigger on its own the widgets will just overflow and not be visible.

The red rectangle shows the maximum size before the label were added.
{{sample: ui_max_size_central_panel}}

## There are many more
How exactly a parent container reacts to a child ui exceeding its maximum size depends entirely on the parent container.
This book cannot document the behavior for every container.

When experiencing weird growth, resizing issues or overflow try to keep the maximum size in mind.
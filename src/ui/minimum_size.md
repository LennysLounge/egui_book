# Minimum size

Ui's have a minimum size.
When adding widgets to a ui, those widgets take up space.
The minimum size is the smallest rectangle that can contain all the widgets.

In this sample i have colored the minimum rectangle in blue to make it visible.
As you add or remove label you can see that the minimum rect of the ui always exactly encompasses the labels.
{{sample: ui_min_size}}

The minimum size is mostly used by parent ui's to know how much space a child ui actually used.
A window for example uses the minimum size to determine the size of the window.
Add or remove labels and see how the window changes its size to match the minimum size of the ui.
{{sample: ui_min_size_window}}

> [!NOTE]
> In the sample above we cannot resize the window even though the resize flag is set to true and the resize handles appear.
> This is because the size of the window is take from the minimum size of the ui while the resizing modified the maximum size of the ui.
> More on how to fix this in the chapter about the maximum size.
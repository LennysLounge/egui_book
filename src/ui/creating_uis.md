# Creating Ui's
Creating your own ui's is simple and allow you greater control over how and where widgets are placed.
Its a great tool to have in your pocket and useful to know to understand various layouts and methods.

This page is no a complete list of all ways to create ui's but is just a collection of usefull techniques.


## Creating scopes
The easiest way to create new ui is to use the `Ui::scope` method.
This creates an exact copy of the parent ui.
Since each instacen of a `Ui` has its own style you can modify the style of a ui without impacting the parent ui.

In this sample we modify the style in a scope.
The seperator lines are not part of the scope.
Scopes are entirely invisible.
{{sample: ui_creating_uis_scope}}

 - ui.push_id
 - ui.scope_builder


## Changing layouts
 - ui.horizontal
 - ui.vertical
 - ui.with_layout

 ## Full control with UiBuilder
 - ui.new_child
 - ui.allocate_ui
 - ui.allocate_ui_with_layout
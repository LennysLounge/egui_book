# Widgets

## What is a widget

Widgets can be very simple or they can be very complicated. In egui a widget is a trait
that you can implement. The trait has only one method called `ui` which takes in a
`&mut Ui` and returns a `Response`.
This trait definition is very free and open. It does not give us any requirements 
for the structure of a widget.
There is also a blanket implementation for `FnOnce(&mut Ui) -> Response` which means that a closure can be a widget.

```rust
{{#include ../rust/src/bin/widget_closure.rs:widget_closure}}
```
The snippet shows how you can use the `Ui::add` method, which is normally reserved
for things that implement `Widget`, with a closure. Inside the closure the add a button and a
label and return the response of the label.

In no way shape or form would i describe the closure as a "widget". It acts more like a container
that groups multiple widgets. 

There is also the `egui::ComboBox` widget. The Combo Box does not implement the `Widget` trait
and it does not have a `ui` method. Instead you construct a new instance of it and call the 
`show_ui` method on it.

```rust
{{#include ../rust/src/bin/widget_closure.rs:combo_box}}
```

A Combo Box is not a widget as far as egui can tell but for all intents and purposes it behaves
exactly like you would expect from a widget.

I hope these examples make it clear that the definition of a "widget" is not very clear in egui.

### Why is a ComboBox not a widget?
There is not technical reason why the ComboBox could not be a widget.

The ComboBox, as it exists in its current state, uses a callback closure to add the contents
to the drop down menu. This sort of design is ubiquitous in egui and is used all over the place and 
therefore you should already be very familiar with this pattern.
If you wanted to implement the `Widget` trait for the combo box you would have to find a way to store
the menu contents closure until the combo box is added to a ui and the `ui` method is called.

You would have to store a box of a FnOnce trait object or store a reference to the closure and deal
with the lifetimes of that decision. It is not an impossible task but it carries some difficulties with it.

You could change the way a combo box is created and used to get around the need to store a closure but
thats not the decision that was made. Clearly the benefits of implementing the `Widget` trait did not outweigh
the disadvantages.

## The ideal widget

As discussed in the previous section, just because something implements the `Widget` trait does not mean
that it matches with our intuitive understanding of what a widget is.
This misunderstanding can cause frustration when the layout of a ui does not work the way you think it should.

In this section i want to introduce the concept of an ideal widget which hopefully aligns much better with
the intuitive understand of a widget. Understanding what an ideal widget is will allow you to recognize 
widgets that are not ideal and the implication their non-ideality has on the layout of a ui.


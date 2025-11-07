# Allocating space

Every instance of a `Ui` has a private field called the placer.
The placer is responsible for deciding the location of widgets in the ui so that they follow the layout of the ui
and so that they dont overlap each other.
When a widget is added to a ui it first has to ask the placer where the widget should be placed.
Since the placer is private you cannot use it directly and instead have to use the provided methods on the `Ui` struct.

You can ask the placer where to put your widget by calling one of the many allocation methods on the ui.
By calling `Ui::allocate_space` you are telling the placer how big your widget is and the 
placer responds by returning you the rectangle where your widget should be rendered and an Id to identify your widget.
```rust
{{#include ../../rust/src/bin/snippets.rs:allocate_space}}
```
If the layout of the ui is justified then the returned rectangle might be bigger than the size you requested. 
The placer will never return a rectangle that is smaller than you requested.
If your widget does not fit in the remaining space of the ui the placer will instead make the ui bigger to accomidate 
your widget.

## Allocating responses

A common thing for widgets to do it to ask the placer for the position of the widget and then immediately check for
interaction in that area. Normally you would first allocate space and then check interactions like this:
```rust
{{#include ../../rust/src/bin/snippets.rs:allocate_and_interact}}
```
This pattern is so common that many allocation methods will check for interaction themselves as a convenience to the caller.

**Ui::allocate_response:**  
`Ui::allocate_response` both allocates the requested size and asks for interaction with the returned space. The id and rect
are naturally part of the response and therefore this method only returns the response.
```rust
{{#include ../../rust/src/bin/snippets.rs:allocate_response}}
```

**Ui::allocate_at_least:**  
`Ui::allocate_at_least` behaves the same as `Ui::allocate_response` but the rect of the response is returned sperately.
The source of `Ui::allocate_atleast` is just two lines and calls `Ui::allocate_response` directly.
```rs
pub fn allocate_at_least(&mut self, desired_size: Vec2, sense: Sense) -> (Rect, Response) {
    let response = self.allocate_response(desired_size, sense);
    (response.rect, response)
}
```

**Ui::allocate_exact_size**  
Some widgets, like a checkbox for example, cannot grow bigger just because they are part of a justified layout.
For these cases the `Ui::allocate_exact_size` method can be usefull.
It allocates a response just like `Ui::allocate_response` would but additionally it also returns a rectangle that
is exactly the size you requested.

Keep in mind that interaction happens in the response rectangle which is the justified size.

{{sample: allocate_exact_size}}

In this sample the widgets are part of vertical layout with horizontal justification.
Any allocated size will therefore expand to the full width.
The red rectangle is the size of the response object.
The blue rectangle is the rectangle returned by `Ui::allocate_exact_size`.

Notice that when you change the corss alignment of the layout the position of the blue rectangle inside the red response rectangle
also changes.

## Allocating rectangles directly and avoiding the layout

You can also skip the layout and tell the placer directly where you want your widget to be placed.
For these methods you dont use the size of the widget but supply a rectangle directly.

**Ui::allocate_rect**  
This just places the rect directly into the layout and returns the response for it.

**Ui::advance_cursor_after_rect**  
This does essentially the same thing as the previous method but does not interact with the rectangle and therfore only returns the Id of the widget.





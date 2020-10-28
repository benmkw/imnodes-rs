# imnodes-rs

![Tests](https://github.com/benmkw/imnodes-rs/workflows/Tests/badge.svg)

big thanks to 4bb4 for [implot-rs](https://github.com/4bb4/implot-rs) !

its working :) ( image is outdated, works even more now ;) )

![](wip.png)

# docs
`cargo doc --no-deps --open`

# TODO/ Ideas
- figure out a better Graph api that is still typesafe and easy to use, revisit this with GAT probably 
- IO
    - all Mouse/ Modifier helpers

nice to have:
- use Serde to make it possible to declare graphs and render them
    - load and save as well using imnode_* functions
- add comments to everything
    - figure out good descriptions of coordinate systems 
- review types in unsafe code

# Example (see `imnodes-wgpu-examples/src/ui.rs`)

```rust
// hello world

pub fn show_hello_world(ui: &Ui, context: &imnodes::EditorContext) {
    let mut id_gen = context.new_identifier_generator();

    editor(&context, |editor| {
        editor.node(id_gen.next_node(), |node| {
            node.add_titlebar(|| {
                ui.text(im_str!("simple node :)"));
            });

            node.add_input(id_gen.next_input_pin(), PinShape::Circle, || {
                ui.text(im_str!("input"));
            });

            node.add_output(id_gen.next_output_pin(), PinShape::QuadFilled, || {
                ui.text(im_str!("output"));
            });
        });
    });
}
```
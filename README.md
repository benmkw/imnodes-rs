# imnodes-rs

![Tests](https://github.com/benmkw/imnodes-rs/workflows/Tests/badge.svg)

These are bindings for [imnodes](https://github.com/Nelarius/imnodes)
using [cimnodes](https://github.com/cimgui/cimnodes) for [imgui-rs](https://github.com/Gekkio/imgui-rs).

They are inspsired by [implot-rs](https://github.com/4bb4/implot-rs).

![example image](example.png)

## docs

`cargo doc --no-deps --open`

## TODO/ Ideas

- add example with salsa or some other incremental computation lib
- IO
    - all Mouse/ Modifier helpers

nice to have:

- use Serde to make it possible to declare graphs and render them
    - load and save as well using imnode_* functions
- add comments to everything
    - figure out good descriptions of coordinate systems
- review types in unsafe code
    - especially -> &mut sys::Style

## Example (see `imnodes-wgpu-examples/src/hello_world.rs`)

```rust
use imgui::{im_str, Ui};
use imnodes::{editor, PinShape};

pub fn show(ui: &Ui, context: &mut imnodes::EditorContext) {
    let mut id_gen = context.new_identifier_generator();

    editor(context, |mut editor| {
        editor.add_node(id_gen.next_node(), |mut node| {
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

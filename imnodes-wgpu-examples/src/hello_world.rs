use imgui::{im_str, Ui};
use imnodes::{editor, PinShape};

/// https://github.com/Nelarius/imnodes/blob/master/example/hello.cpp
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

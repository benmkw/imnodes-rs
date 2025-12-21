/// https://github.com/Nelarius/imnodes/blob/master/example/hello.cpp
pub fn show<'a>(ui: &imgui::Ui, context: &mut imnodes::EditorContext<'a>) {
    let mut id_generator = context.new_identifier_generator();

    imnodes::editor(context, |mut editor| {
        editor.add_node(id_generator.next_node(), |mut node| {
            node.add_titlebar(|| ui.text("simple node :)"));

            node.add_input(
                &imnodes::InputAttributeDesc {
                    id: id_generator.next_input_pin(),
                    shape: imnodes::PinShape::Circle,
                },
                || ui.text("input"),
            );

            node.add_output(
                &imnodes::OutputAttributeDesc {
                    id: id_generator.next_output_pin(),
                    shape: imnodes::PinShape::QuadFilled,
                },
                || ui.text("output"),
            );
        });
    });
}

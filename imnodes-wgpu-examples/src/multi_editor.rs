use imnodes::{
    editor, AttributeFlag, AttributeId, Context, EditorContext, IdentifierGenerator, InputPinId,
    LinkId, NodeId, OutputPinId, PinShape,
};

pub struct MultiEditState {
    editor_context: EditorContext,
    id_gen: IdentifierGenerator,

    nodes: Vec<Node>,
    links: Vec<Link>,

    style: imnodes::ImNodesStyle,
}

struct Link {
    id: LinkId,
    start: OutputPinId,
    end: InputPinId,
}
struct Node {
    id: NodeId,
    input: InputPinId,
    output: OutputPinId,
    attribute: AttributeId,
    value: f32,
}

impl MultiEditState {
    pub fn new(context: &Context) -> Self {
        let editor_context = context.create_editor();
        let id_gen = editor_context.new_identifier_generator();

        Self {
            id_gen,
            editor_context,
            nodes: vec![],
            links: vec![],
            style: imnodes::create_imnodes_style(),
        }
    }
}

/// https://github.com/Nelarius/imnodes/blob/master/example/multi_editor.cpp
pub fn show(ui: &imgui::Ui, state: &mut MultiEditState) {
    // just as an example, should not be needed anymore
    // see https://github.com/ocornut/imgui/blob/master/docs/FAQ.md#q-how-can-i-have-multiple-widgets-with-the-same-label

    // TODO HACK why was this needed? is it safe to remove this?
    let id = ui.push_id(format!(
        "{:?}",
        core::ptr::from_mut::<MultiEditState>(state)
    ));

    state
        .editor_context
        .set_style_colors_classic(&mut state.style);

    let on_snap = state
        .editor_context
        .push(AttributeFlag::EnableLinkCreationOnSnap);
    let detach = state
        .editor_context
        .push(AttributeFlag::EnableLinkDetachWithDragClick);

    let MultiEditState {
        ref mut editor_context,
        nodes,
        links,
        id_gen,
        ..
    } = state;

    if ui.button("Add a Node") {
        nodes.push(Node {
            id: id_gen.next_node(),
            input: id_gen.next_input_pin(),
            output: id_gen.next_output_pin(),
            value: 0.0,
            attribute: id_gen.next_attribute(),
        });
    }

    ui.same_line();

    ui.text("or you can press \"A\" or right click");

    let outer_scope = editor(editor_context, |mut editor| {
        if editor.is_hovered()
            && (ui.is_key_released(imgui::Key::A) || ui.is_mouse_clicked(imgui::MouseButton::Right))
        {
            let id = id_gen.next_node();
            let [x, y] = ui.io().mouse_pos;
            let _ = id.set_position(x, y, imnodes::CoordinateSystem::ScreenSpace);
            nodes.push(Node {
                id,
                input: id_gen.next_input_pin(),
                output: id_gen.next_output_pin(),
                value: 0.0,
                attribute: id_gen.next_attribute(),
            });
        }

        for curr_node in nodes.iter_mut() {
            editor.add_node(curr_node.id, |mut node| {
                node.add_titlebar(|| {
                    ui.text("node");
                });

                node.add_input(curr_node.input, PinShape::QuadFilled, || {
                    ui.text("input");
                });

                node.attribute(curr_node.attribute, || {
                    ui.set_next_item_width(130.0);
                    ui.slider_config("value", 0.0, 10.0)
                        .display_format(format!("{:.2}", curr_node.value))
                        .build(&mut curr_node.value);
                });

                node.add_output(curr_node.output, PinShape::CircleFilled, || {
                    ui.text("output");
                });
            });
        }

        for Link { id, start, end } in links {
            editor.add_link(*id, *end, *start);
        }
    });

    if let Some(link) = outer_scope.links_created() {
        state.links.push(Link {
            id: state.id_gen.next_link(),
            start: link.start_pin,
            end: link.end_pin,
        });
    }

    if let Some(link) = outer_scope.get_dropped_link() {
        state
            .links
            .swap_remove(state.links.iter().position(|e| e.id == link).unwrap());
    }

    on_snap.pop();
    detach.pop();
    id.pop();
}

use imnodes::{
    AttributeFlags, AttributeId, Context, EditorContext, IdentifierGenerator, InputPinId, LinkId,
    NodeId, OutputPinId, PinShape, Style, editor,
};

pub struct MultiEditState {
    pub editor_context: EditorContext,
    id_gen: IdentifierGenerator,
    nodes: Vec<Node>,
    links: Vec<Link>,
    style: Style,
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
            style: Style::default(),
        }
    }
}

/// https://github.com/Nelarius/imnodes/blob/master/example/multi_editor.cpp
pub fn show(ui: &imgui::Ui, state: &mut MultiEditState) {
    // Push unique ID for this editor instance using push_id_ptr with a reference
    let id = ui.push_id_ptr(&state.editor_context);

    state
        .editor_context
        .set_style_colors_classic(&mut state.style);

    let on_snap = state
        .editor_context
        .push_attribute_flag(AttributeFlags::EnableLinkCreationOnSnap);
    let detach = state
        .editor_context
        .push_attribute_flag(AttributeFlags::EnableLinkDetachWithDragClick);

    let MultiEditState {
        editor_context,
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

        // Iterate using indices to allow mutable borrow inside slider closure
        for i in 0..nodes.len() {
            let node_id = nodes[i].id;
            let input_pin = nodes[i].input;
            let output_pin = nodes[i].output;
            let attr_id = nodes[i].attribute;

            editor.add_node(node_id, |mut node_scope| {
                node_scope.add_titlebar(|| {
                    ui.text("node");
                });

                node_scope.add_input(input_pin, PinShape::QuadFilled, || {
                    ui.text("input");
                });

                node_scope.add_static_attribute(attr_id, || {
                    ui.set_next_item_width(130.0);
                    // Get mutable access only for the slider build call
                    if let Some(node_mut) = nodes.get_mut(i) {
                        ui.slider_config("value", 0.0, 10.0)
                            .display_format(format!("{:.2}", node_mut.value))
                            .build(&mut node_mut.value);
                    }
                });

                node_scope.add_output(output_pin, PinShape::CircleFilled, || {
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

    if let Some(link_id) = outer_scope.get_destroyed_link() {
        state.links.retain(|link| link.id != link_id);
    }

    on_snap.pop();
    detach.pop();
    id.pop();
}

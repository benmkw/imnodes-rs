use imnodes::{
    AttributeFlags, AttributeId, Context, EditorContext, IdentifierGenerator, InputAttributeDesc,
    InputPinId, LinkId, MiniMapDesc, MiniMapLocation, NodeId, OutputAttributeDesc, OutputPinId,
    PinShape, StaticAttributeDesc, StyleDesc, editor,
};

pub struct MultiEditState<'a> {
    pub editor_context: EditorContext<'a>,
    id_gen: IdentifierGenerator,
    pass_nodes: Vec<Node>,
    pass_links: Vec<Link>,
    resource_nodes: Vec<Node>,
    resource_links: Vec<Link>,
    positions_initialized: bool,
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

impl<'a> MultiEditState<'a> {
    pub fn new(context: &'a Context) -> Self {
        let editor_context = context.create_editor(&imnodes::EditorContextDesc::default());
        let mut id_gen = editor_context.new_identifier_generator();

        // Create initial demo nodes
        let pass1_id = id_gen.next_node();
        let pass2_id = id_gen.next_node();

        let pass_nodes = vec![
            Node {
                id: pass1_id,
                input: id_gen.next_input_pin(),
                output: id_gen.next_output_pin(),
                value: 0.5,
                attribute: id_gen.next_attribute(),
            },
            Node {
                id: pass2_id,
                input: id_gen.next_input_pin(),
                output: id_gen.next_output_pin(),
                value: 0.3,
                attribute: id_gen.next_attribute(),
            },
        ];

        let resource1_id = id_gen.next_node();
        let resource2_id = id_gen.next_node();

        let resource_nodes = vec![
            Node {
                id: resource1_id,
                input: id_gen.next_input_pin(),
                output: id_gen.next_output_pin(),
                value: 0.7,
                attribute: id_gen.next_attribute(),
            },
            Node {
                id: resource2_id,
                input: id_gen.next_input_pin(),
                output: id_gen.next_output_pin(),
                value: 0.9,
                attribute: id_gen.next_attribute(),
            },
        ];

        let pass_links = vec![Link {
            id: id_gen.next_link(),
            start: pass_nodes[0].output,
            end: pass_nodes[1].input,
        }];

        let resource_links = vec![Link {
            id: id_gen.next_link(),
            start: resource_nodes[0].output,
            end: resource_nodes[1].input,
        }];

        Self {
            id_gen,
            editor_context,
            pass_nodes,
            pass_links,
            resource_nodes,
            resource_links,
            positions_initialized: false,
        }
    }
}

/// https://github.com/Nelarius/imnodes/blob/master/example/multi_editor.cpp
pub fn show<'a>(ui: &imgui::Ui, state: &mut MultiEditState<'a>) {
    // Push unique ID for this editor instance using push_id_ptr with a reference
    let id = ui.push_id_ptr(&state.editor_context);

    // Set as current editor before using context methods outside editor()
    let _ = state.editor_context.set_as_current_editor();

    let MultiEditState {
        editor_context,
        pass_nodes,
        pass_links,
        resource_nodes,
        resource_links,
        id_gen,
        ..
    } = state;

    // Logic previously inside .run()
    {
        if ui.button("Add Pass Node") {
            pass_nodes.push(Node {
                id: id_gen.next_node(),
                input: id_gen.next_input_pin(),
                output: id_gen.next_output_pin(),
                value: 0.0,
                attribute: id_gen.next_attribute(),
            });
        }

        ui.same_line();

        if ui.button("Add Resource Node") {
            resource_nodes.push(Node {
                id: id_gen.next_node(),
                input: id_gen.next_input_pin(),
                output: id_gen.next_output_pin(),
                value: 0.0,
                attribute: id_gen.next_attribute(),
            });
        }

        ui.same_line();

        ui.text("or you can press \"A\" or right click");

        let outer_scope = create_the_editor(
            ui,
            editor_context, // Use the context from the closure
            pass_nodes,
            pass_links,
            resource_nodes,
            resource_links,
            id_gen,
            &mut state.positions_initialized,
        );

        if let Some(link) = outer_scope.links_created() {
            // Simple logic to decide where to add the link:
            // If the start pin belongs to a pass node, add to pass_links.
            // Otherwise, add to resource_links.
            // This is a simplification; a more robust solution would check both start and end pins.
            if pass_nodes.iter().any(|node| node.output == link.start_pin) {
                pass_links.push(Link {
                    id: id_gen.next_link(),
                    start: link.start_pin,
                    end: link.end_pin,
                });
            } else {
                resource_links.push(Link {
                    id: id_gen.next_link(),
                    start: link.start_pin,
                    end: link.end_pin,
                });
            }
        }

        if let Some(link_id) = outer_scope.get_destroyed_link() {
            pass_links.retain(|link| link.id != link_id);
            resource_links.retain(|link| link.id != link_id);
        }
    }

    id.pop();
}

/// main node ui
fn create_the_editor<'a>(
    ui: &imgui::Ui,
    editor_context: &mut EditorContext<'a>,
    pass_nodes: &mut Vec<Node>,
    pass_links: &Vec<Link>,
    resource_nodes: &mut Vec<Node>,
    resource_links: &Vec<Link>,
    id_gen: &mut IdentifierGenerator,
    positions_initialized: &mut bool,
) -> imnodes::OuterScope {
    editor(editor_context, |mut editor| {
        // Push flags that were previously in show()
        editor.push_style(
            &StyleDesc {
                flags: &[
                    AttributeFlags::EnableLinkCreationOnSnap,
                    AttributeFlags::EnableLinkDetachWithDragClick,
                ],
                ..Default::default()
            },
            |editor| {
                // Set initial positions for demo nodes (only once)
                if !*positions_initialized {
                    if pass_nodes.len() >= 2 {
                        let _ = pass_nodes[0].id.set_position(
                            100.0,
                            80.0,
                            imnodes::CoordinateSystem::EditorSpace,
                        );
                        let _ = pass_nodes[1].id.set_position(
                            100.0,
                            220.0,
                            imnodes::CoordinateSystem::EditorSpace,
                        );
                    }
                    if resource_nodes.len() >= 2 {
                        let _ = resource_nodes[0].id.set_position(
                            450.0,
                            80.0,
                            imnodes::CoordinateSystem::EditorSpace,
                        );
                        let _ = resource_nodes[1].id.set_position(
                            450.0,
                            220.0,
                            imnodes::CoordinateSystem::EditorSpace,
                        );
                    }
                    *positions_initialized = true;
                }

                editor.add_mini_map(&MiniMapDesc {
                    size_fraction: 0.2,
                    location: MiniMapLocation::BottomLeft,
                });

                // Context menu logic
                if editor.is_hovered()
                    && (ui.is_mouse_clicked(imgui::MouseButton::Right)
                        || ui.is_key_released(imgui::Key::A))
                {
                    ui.open_popup("popup_add_node");
                }

                if let Some(_popup) = ui.begin_popup("popup_add_node") {
                    let click_pos = ui.io().mouse_pos;

                    if ui.menu_item("Add Pass Node") {
                        let node_id = id_gen.next_node();
                        let _ = node_id.set_position(
                            click_pos[0],
                            click_pos[1],
                            imnodes::CoordinateSystem::ScreenSpace,
                        );
                        pass_nodes.push(Node {
                            id: node_id,
                            input: id_gen.next_input_pin(),
                            output: id_gen.next_output_pin(),
                            value: 0.0,
                            attribute: id_gen.next_attribute(),
                        });
                    }

                    if ui.menu_item("Add Resource Node") {
                        let node_id = id_gen.next_node();
                        let _ = node_id.set_position(
                            click_pos[0],
                            click_pos[1],
                            imnodes::CoordinateSystem::ScreenSpace,
                        );
                        resource_nodes.push(Node {
                            id: node_id,
                            input: id_gen.next_input_pin(),
                            output: id_gen.next_output_pin(),
                            value: 0.0,
                            attribute: id_gen.next_attribute(),
                        });
                    }
                }

                // Pass nodes styling (scoped)
                editor.push_style(
                    &StyleDesc {
                        colors: &[
                            (
                                imnodes::ColorStyle::TitleBar,
                                [0.62, 0.41, 0.31, 0.70].into(),
                            ),
                            (
                                imnodes::ColorStyle::TitleBarHovered,
                                [0.98, 0.52, 0.26, 0.80].into(),
                            ),
                            (
                                imnodes::ColorStyle::TitleBarSelected,
                                [0.98, 0.52, 0.26, 1.0].into(), // Fixed alpha
                            ),
                            (imnodes::ColorStyle::Pin, [0.62, 0.41, 0.31, 0.70].into()),
                            (
                                imnodes::ColorStyle::PinHovered,
                                [0.98, 0.52, 0.26, 0.80].into(),
                            ),
                            (imnodes::ColorStyle::Link, [0.62, 0.41, 0.31, 0.70].into()),
                            (
                                imnodes::ColorStyle::LinkHovered,
                                [0.98, 0.52, 0.26, 0.80].into(),
                            ),
                            (
                                imnodes::ColorStyle::LinkSelected,
                                [0.98, 0.52, 0.26, 1.0].into(), // Fixed alpha
                            ),
                        ],
                        ..Default::default()
                    },
                    |editor| {
                        // Demonstrates per-node style push/pop using the real NodeScope::push_style API
                        for i in 0..pass_nodes.len() {
                            let node_id = pass_nodes[i].id;
                            let input_pin = pass_nodes[i].input;
                            let output_pin = pass_nodes[i].output;
                            let attr_id = pass_nodes[i].attribute;

                            let is_even = i % 2 == 0;
                            let node_style = if is_even {
                                StyleDesc {
                                    colors: &[(
                                        imnodes::ColorStyle::TitleBar,
                                        [0.8, 0.3, 0.3, 1.0].into(),
                                    )],
                                    ..Default::default()
                                }
                            } else {
                                StyleDesc {
                                    colors: &[(
                                        imnodes::ColorStyle::TitleBar,
                                        [0.3, 0.3, 0.8, 1.0].into(),
                                    )],
                                    ..Default::default()
                                }
                            };

                            editor.add_node(node_id, |mut node_scope| {
                                node_scope.push_style(&node_style, |node_scope| {
                                    node_scope.add_titlebar(|| {
                                        ui.text(format!("pass node {}", i));
                                    });
                                    node_scope.add_input(
                                        &InputAttributeDesc {
                                            id: input_pin,
                                            shape: PinShape::QuadFilled,
                                        },
                                        || {
                                            ui.text("input");
                                        },
                                    );
                                    node_scope.add_static_attribute(
                                        &StaticAttributeDesc { id: attr_id },
                                        || {
                                            ui.set_next_item_width(130.0);
                                            if let Some(node_mut) = pass_nodes.get_mut(i) {
                                                ui.slider_config("value", 0.0, 10.0)
                                                    .display_format(format!(
                                                        "{:.2}",
                                                        node_mut.value
                                                    ))
                                                    .build(&mut node_mut.value);
                                            }
                                        },
                                    );
                                    node_scope.add_output(
                                        &OutputAttributeDesc {
                                            id: output_pin,
                                            shape: PinShape::CircleFilled,
                                        },
                                        || {
                                            ui.text("output");
                                        },
                                    );
                                });
                            });
                        }

                        for Link { id, start, end } in pass_links.iter() {
                            editor.add_link(*id, *end, *start);
                        }
                    },
                );

                // Resource nodes styling
                editor.push_style(
                    &StyleDesc {
                        colors: &[
                            (
                                imnodes::ColorStyle::TitleBar,
                                [0.31, 0.41, 0.62, 0.70].into(),
                            ),
                            (
                                imnodes::ColorStyle::TitleBarHovered,
                                [0.26, 0.52, 0.98, 0.80].into(),
                            ),
                            (
                                imnodes::ColorStyle::TitleBarSelected,
                                [0.26, 0.52, 0.98, 1.0].into(),
                            ),
                            (imnodes::ColorStyle::Pin, [0.31, 0.41, 0.62, 0.70].into()),
                            (
                                imnodes::ColorStyle::PinHovered,
                                [0.26, 0.52, 0.98, 0.80].into(),
                            ),
                            (imnodes::ColorStyle::Link, [0.31, 0.41, 0.62, 0.70].into()),
                            (
                                imnodes::ColorStyle::LinkHovered,
                                [0.26, 0.52, 0.98, 0.80].into(),
                            ),
                            (
                                imnodes::ColorStyle::LinkSelected,
                                [0.26, 0.52, 0.98, 1.0].into(),
                            ),
                        ],
                        ..Default::default()
                    },
                    |editor| {
                        for i in 0..resource_nodes.len() {
                            let node_id = resource_nodes[i].id;
                            let input_pin = resource_nodes[i].input;
                            let output_pin = resource_nodes[i].output;
                            let attr_id = resource_nodes[i].attribute;

                            let is_even = i % 2 == 0;
                            let node_style = if is_even {
                                StyleDesc {
                                    colors: &[(
                                        imnodes::ColorStyle::TitleBar,
                                        [0.2, 0.7, 0.4, 1.0].into(),
                                    )],
                                    ..Default::default()
                                }
                            } else {
                                StyleDesc {
                                    colors: &[(
                                        imnodes::ColorStyle::TitleBar,
                                        [0.4, 0.7, 0.2, 1.0].into(),
                                    )],
                                    ..Default::default()
                                }
                            };

                            editor.add_node(node_id, |mut node_scope| {
                                node_scope.push_style(&node_style, |node_scope| {
                                    node_scope.add_titlebar(|| {
                                        ui.text(format!("resource node {}", i));
                                    });
                                    node_scope.add_input(
                                        &InputAttributeDesc {
                                            id: input_pin,
                                            shape: PinShape::QuadFilled,
                                        },
                                        || {
                                            ui.text("input");
                                        },
                                    );
                                    node_scope.add_static_attribute(
                                        &StaticAttributeDesc { id: attr_id },
                                        || {
                                            ui.set_next_item_width(130.0);
                                            if let Some(node_mut) = resource_nodes.get_mut(i) {
                                                ui.slider_config("value", 0.0, 10.0)
                                                    .display_format(format!(
                                                        "{:.2}",
                                                        node_mut.value
                                                    ))
                                                    .build(&mut node_mut.value);
                                            }
                                        },
                                    );
                                    node_scope.add_output(
                                        &OutputAttributeDesc {
                                            id: output_pin,
                                            shape: PinShape::CircleFilled,
                                        },
                                        || {
                                            ui.text("output");
                                        },
                                    );
                                });
                            });
                        }

                        for Link { id, start, end } in resource_links.iter() {
                            editor.add_link(*id, *end, *start);
                        }
                    },
                );
            },
        );
    })
}

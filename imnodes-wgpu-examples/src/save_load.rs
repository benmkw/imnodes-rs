use imgui::Key;
use imnodes::{
    AttributeId, Context, EditorContext, EditorContextDesc, IdentifierGenerator,
    InputAttributeDesc, InputPinId, LinkId, NodeId, OutputAttributeDesc, OutputPinId, PinShape,
    StaticAttributeDesc, editor,
};
use std::fs::File;
use std::io::{Read, Write};
use std::mem;
use std::slice;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Node {
    id: NodeId,
    input: InputPinId,
    output: OutputPinId,
    attribute: AttributeId,
    value: f32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct Link {
    id: LinkId,
    start_attr: OutputPinId,
    end_attr: InputPinId,
}

#[derive(Debug)]
pub struct SaveLoadState<'a> {
    pub editor_context: EditorContext<'a>,
    id_gen: IdentifierGenerator,
    nodes: Vec<Node>,
    links: Vec<Link>,
    file_content: String,
    positions_initialized: bool,
}

impl<'a> SaveLoadState<'a> {
    pub fn new(context: &'a Context) -> Self {
        let editor_context = context.create_editor(&EditorContextDesc::default());
        let mut id_gen = editor_context.new_identifier_generator();

        // Create demo nodes
        let node1 = Node {
            id: id_gen.next_node(),
            input: id_gen.next_input_pin(),
            output: id_gen.next_output_pin(),
            attribute: id_gen.next_attribute(),
            value: 2.5,
        };
        let node2 = Node {
            id: id_gen.next_node(),
            input: id_gen.next_input_pin(),
            output: id_gen.next_output_pin(),
            attribute: id_gen.next_attribute(),
            value: 7.3,
        };
        let node3 = Node {
            id: id_gen.next_node(),
            input: id_gen.next_input_pin(),
            output: id_gen.next_output_pin(),
            attribute: id_gen.next_attribute(),
            value: 4.1,
        };

        // Create demo links
        let link1 = Link {
            id: id_gen.next_link(),
            start_attr: node1.output,
            end_attr: node2.input,
        };
        let link2 = Link {
            id: id_gen.next_link(),
            start_attr: node1.output,
            end_attr: node3.input,
        };

        SaveLoadState {
            editor_context,
            id_gen,
            nodes: vec![node1, node2, node3],
            links: vec![link1, link2],
            file_content: String::new(),
            positions_initialized: false,
        }
    }

    fn save(&mut self) {
        // Save the internal imnodes state
        let _ = self
            .editor_context
            .save_current_editor_state_to_file("save_load.ini");

        // Dump our editor state as bytes into a file
        if let Ok(mut file) = File::create("save_load.bytes") {
            let num_nodes = self.nodes.len();
            let _ = file.write_all(&num_nodes.to_ne_bytes());

            // Safety: Writing raw struct bytes. Structs are #[repr(C)] and contain only primitives/repr(C) fields.
            let nodes_bytes = unsafe {
                slice::from_raw_parts(
                    self.nodes.as_ptr() as *const u8,
                    num_nodes * mem::size_of::<Node>(),
                )
            };
            let _ = file.write_all(nodes_bytes);

            let num_links = self.links.len();
            let _ = file.write_all(&num_links.to_ne_bytes());

            let links_bytes = unsafe {
                slice::from_raw_parts(
                    self.links.as_ptr() as *const u8,
                    num_links * mem::size_of::<Link>(),
                )
            };
            let _ = file.write_all(links_bytes);
        }

        // Update file content display
        self.update_file_content_display();
    }

    fn load(&mut self) {
        // Load the internal imnodes state
        let _ = self
            .editor_context
            .load_current_editor_state_from_file("save_load.ini");

        // Load our editor state into memory
        if let Ok(mut file) = File::open("save_load.bytes") {
            let mut usize_buf = [0u8; mem::size_of::<usize>()];

            // Read nodes
            if file.read_exact(&mut usize_buf).is_ok() {
                let num_nodes = usize::from_ne_bytes(usize_buf);
                // We need placeholders to resize the vector.
                // Using new IDs is safe because they will be overwritten by read_exact.
                // It is a bit wasteful of the *current* id_gen, but that one is about to be reset anyway.
                if self.nodes.len() < num_nodes {
                    self.nodes.reserve(num_nodes - self.nodes.len());
                }
                // We can't use resize because we don't have a cheap Clone/Copy default constructor for NodeId etc
                // without using the generator.
                // Instead, we clear and push new placeholder nodes.
                self.nodes.clear();
                for _ in 0..num_nodes {
                    self.nodes.push(Node {
                        id: self.id_gen.next_node(),
                        input: self.id_gen.next_input_pin(),
                        output: self.id_gen.next_output_pin(),
                        attribute: self.id_gen.next_attribute(),
                        value: 0.0,
                    });
                }

                let nodes_bytes = unsafe {
                    slice::from_raw_parts_mut(
                        self.nodes.as_mut_ptr() as *mut u8,
                        num_nodes * mem::size_of::<Node>(),
                    )
                };
                let _ = file.read_exact(nodes_bytes);
            }

            // Read links
            if file.read_exact(&mut usize_buf).is_ok() {
                let num_links = usize::from_ne_bytes(usize_buf);
                self.links.clear();
                // Create placeholders (values don't matter, will be overwritten)
                // We reuse one valid ID set or generate new ones.
                // To avoid exhausting generator (though unlikely for demo), we can just panic if fails or whatever.
                // But simplified:
                let dummy_node = self.nodes.first().cloned().unwrap_or(Node {
                    id: self.id_gen.next_node(),
                    input: self.id_gen.next_input_pin(),
                    output: self.id_gen.next_output_pin(),
                    attribute: self.id_gen.next_attribute(),
                    value: 0.0,
                });

                for _ in 0..num_links {
                    self.links.push(Link {
                        id: self.id_gen.next_link(),
                        start_attr: dummy_node.output,
                        end_attr: dummy_node.input,
                    });
                }

                let links_bytes = unsafe {
                    slice::from_raw_parts_mut(
                        self.links.as_mut_ptr() as *mut u8,
                        num_links * mem::size_of::<Link>(),
                    )
                };
                let _ = file.read_exact(links_bytes);
            }

            // Reconstruct IdentifierGenerator state
            // We start with a fresh generator (0,0,0) and register all loaded IDs.
            self.id_gen = self.editor_context.new_identifier_generator();

            for node in &self.nodes {
                self.id_gen.register_node_id(&node.id);
                self.id_gen.register_input_pin_id(&node.input);
                self.id_gen.register_output_pin_id(&node.output);
                self.id_gen.register_attribute_id(&node.attribute);
            }
            for link in &self.links {
                self.id_gen.register_link_id(&link.id);
            }
        }

        // Update file content display
        self.update_file_content_display();
    }

    fn update_file_content_display(&mut self) {
        let mut content = String::from("---\x20save_load.ini---\n");
        if let Ok(ini_str) = std::fs::read_to_string("save_load.ini") {
            content.push_str(&ini_str);
        } else {
            content.push_str("(not found)\n");
        }

        content.push_str("\n--- save_load.bytes (hexdump partial) ---\n");
        if let Ok(bytes) = std::fs::read("save_load.bytes") {
            for (i, byte) in bytes.iter().take(256).enumerate() {
                if i > 0 && i % 16 == 0 {
                    content.push('\n');
                }
                content.push_str(&format!("{:02X} ", byte));
            }
            if bytes.len() > 256 {
                content.push_str("\n... (truncated)");
            }
        } else {
            content.push_str("(not found)");
        }

        self.file_content = content;
    }
}

pub fn show<'a>(ui: &imgui::Ui, state: &mut SaveLoadState<'a>) {
    ui.text_colored([0.3, 0.8, 0.3, 1.0], "💾 Persistent Node Editor");
    ui.separator();
    ui.text("Press 'A' to add a node at your cursor position");
    ui.text("Drag nodes around, connect them, then close and reopen!");
    ui.spacing();

    if ui.button("💾 Save State") {
        state.save();
    }
    ui.same_line();
    if ui.button("📂 Load State") {
        state.load();
    }
    ui.same_line();
    ui.text_disabled(format!(
        "({} nodes, {} links)",
        state.nodes.len(),
        state.links.len()
    ));

    if !state.file_content.is_empty() && imgui::CollapsingHeader::new("File Content").build(ui) {
        ui.text_wrapped(&state.file_content);
    }

    // Interaction / Debug Panel to cover more APIs
    if imgui::CollapsingHeader::new("Interaction API Coverage").build(ui) {
        let selected_nodes = state.editor_context.get_selected_nodes();
        ui.text(format!("Selected Nodes: {:?}", selected_nodes));

        if let Some(first_id) = selected_nodes.first() {
            let id = *first_id;
            if ui.button("Move Editor to Selected") {
                let _ = id.move_editor_to();
            }
            ui.same_line();
            if ui.button("Snap Selected to Grid") {
                let _ = id.snap_to_grid();
            }
            ui.text(format!("Dimensions: {:?}", id.get_dimensions()));
        } else {
            ui.text_disabled("(Select a node to test API)");
        }

        if ui.button("Clear Selection") {
            state.editor_context.clear_node_selection();
            state.editor_context.clear_link_selection();
        }
        ui.same_line();
        if ui.button("Reset Panning") {
            state.editor_context.reset_panning([0.0, 0.0]);
        }
    }

    let _ = state.editor_context.set_as_current_editor();

    let outer_scope = editor(&state.editor_context, |mut editor_scope| {
        // Set initial positions for demo nodes (only once)
        if !state.positions_initialized && state.nodes.len() >= 3 {
            let _ = state.nodes[0].id.set_position(
                100.0,
                100.0,
                imnodes::CoordinateSystem::EditorSpace,
            );
            let _ =
                state.nodes[1]
                    .id
                    .set_position(350.0, 80.0, imnodes::CoordinateSystem::EditorSpace);
            let _ = state.nodes[2].id.set_position(
                350.0,
                220.0,
                imnodes::CoordinateSystem::EditorSpace,
            );
            state.positions_initialized = true;
        }

        // Handle 'A' key to add node
        if editor_scope.is_hovered() && ui.is_key_released(Key::A) {
            let node_id = state.id_gen.next_node();
            let _ = node_id.set_position(
                ui.io().mouse_pos[0],
                ui.io().mouse_pos[1],
                imnodes::CoordinateSystem::ScreenSpace,
            );
            state.nodes.push(Node {
                id: node_id,
                input: state.id_gen.next_input_pin(),
                output: state.id_gen.next_output_pin(),
                attribute: state.id_gen.next_attribute(),
                value: 0.0,
            });
        }

        for node in &mut state.nodes {
            editor_scope.add_node(node.id, |mut node_scope| {
                node_scope.add_titlebar(|| {
                    ui.text("node");
                });

                node_scope.add_input(
                    &InputAttributeDesc {
                        id: node.input,
                        shape: PinShape::CircleFilled,
                    },
                    || {
                        ui.text("input");
                    },
                );

                node_scope.add_static_attribute(
                    &StaticAttributeDesc { id: node.attribute },
                    || {
                        ui.set_next_item_width(120.0);
                        imgui::Drag::new("value")
                            .speed(0.01)
                            .build(ui, &mut node.value);
                    },
                );

                node_scope.add_output(
                    &OutputAttributeDesc {
                        id: node.output,
                        shape: PinShape::CircleFilled,
                    },
                    || {
                        // Indent to align right - simplified for Rust
                        ui.text("output");
                    },
                );
            });
        }

        for link in &state.links {
            editor_scope.add_link(link.id, link.end_attr, link.start_attr);
        }
    });

    if let Some(link) = outer_scope.links_created() {
        state.links.push(Link {
            id: state.id_gen.next_link(),
            start_attr: link.start_pin,
            end_attr: link.end_pin,
        });
    }

    if let Some(destroyed_link_id) = outer_scope.get_destroyed_link() {
        state.links.retain(|l| l.id != destroyed_link_id);
    }
}

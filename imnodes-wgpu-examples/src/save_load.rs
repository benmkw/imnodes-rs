use imnodes::{
    Context, CoordinateSystem, EditorContext, IdentifierGenerator, InputPinId, LinkId, NodeId,
    OutputPinId, PinShape, editor,
};

// WARNING! this file is not finished yet/ save load does not work yet

#[derive(Clone, Debug)]
struct AppNode {
    id: NodeId,
    input: InputPinId,
    output: OutputPinId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AppLink {
    id: LinkId,
    start_pin: OutputPinId,
    end_pin: InputPinId,
}

#[derive(Debug)]
pub struct SaveLoadState {
    pub editor_context: EditorContext,
    id_gen: IdentifierGenerator,
    nodes: Vec<AppNode>,
    links: Vec<AppLink>,
    saved_state_string: Option<String>,
    status: String,
    last_selected_nodes: Vec<NodeId>,
    last_selected_links: Vec<LinkId>,
    add_node_at: Option<[f32; 2]>,
}

impl SaveLoadState {
    pub fn new(context: &Context) -> Self {
        let editor_context = context.create_editor();
        let id_gen = editor_context.new_identifier_generator();
        SaveLoadState {
            editor_context,
            id_gen,
            nodes: vec![],
            links: vec![],
            saved_state_string: None,
            status: "Ready".to_string(),
            last_selected_nodes: vec![],
            last_selected_links: vec![],
            add_node_at: None,
        }
    }

    // Helper function to add a node
    fn add_node(&mut self, position: Option<[f32; 2]>) {
        let node_id = self.id_gen.next_node();
        let new_node = AppNode {
            id: node_id,
            input: self.id_gen.next_input_pin(),
            output: self.id_gen.next_output_pin(),
        };

        // Set position *before* pushing the node, so it's placed correctly on the first frame
        if let Some(pos) = position {
            let _ = node_id.set_position(pos[0], pos[1], CoordinateSystem::ScreenSpace);
        } else {
            // Place the new node near the center of the screen, adjusted by panning.
            let pan = self.editor_context.get_panning();
            // Estimate center - requires access to window size, which we don't have here easily.
            // Place relative to pan for now. A better approach might involve passing window size.
            let node_x = pan.x + 100.0; // Offset from top-left visible corner
            let node_y = pan.y + 100.0;
            let _ = node_id.set_position(node_x, node_y, CoordinateSystem::GridSpace);
        }

        self.nodes.push(new_node);
        self.status = format!("Added Node {node_id:?}");
    }
}

pub fn show(ui: &imgui::Ui, state: &mut SaveLoadState) {
    let editor_id = ui.push_id_ptr(&state.editor_context);
    let _ = state.editor_context.set_as_current_editor();

    if ui.button("Add Node") {
        state.add_node_at = Some([-1.0, -1.0]); // Sentinel value
    }
    ui.same_line();
    if ui.button("Remove Selected Nodes") {
        if !state.last_selected_nodes.is_empty() {
            let mut removed_count = 0;
            let nodes_to_remove = state.last_selected_nodes.clone();
            state.nodes.retain(|node| {
                if nodes_to_remove.contains(&node.id) {
                    state
                        .links
                        .retain(|link| node.input != link.end_pin && node.output != link.start_pin);
                    removed_count += 1;
                    false
                } else {
                    true
                }
            });
            state.status = format!("Removed {removed_count} node(s)");
            state.editor_context.clear_node_selection();
            state.last_selected_nodes.clear();
        } else {
            state.status = "No nodes selected to remove".to_string();
        }
    }
    ui.same_line();
    ui.text("or press \"A\" / right-click");

    ui.separator();

    ui.text("Save/Load:");
    if ui.button("Save to String") {
        match state.editor_context.save_current_editor_state_to_string() {
            Some(saved_str) => {
                state.saved_state_string = Some(saved_str);
                state.status = "Saved state to internal string".to_string();
            }
            None => {
                state.status = "Failed to save state to string".to_string();
            }
        }
    }
    ui.same_line();
    if ui.button("Load from String") {
        if let Some(saved_str) = &state.saved_state_string {
            // Load the imnodes internal state
            state
                .editor_context
                .load_current_editor_state_from_string(saved_str);
            state.last_selected_nodes.clear();
            state.last_selected_links.clear();
            state.status =
                "Loaded imnodes state from string. App state assumed to match.".to_string();
        } else {
            state.status = "No saved string state to load".to_string();
        }
    }
    // ui.same_line();
    // if ui.button("Save to File") {
    //     match state
    //         .editor_context
    //         .save_current_editor_state_to_file("save_load_state.ini")
    //     {
    //         Ok(_) => state.status = "Saved state to save_load_state.ini".to_string(),
    //         Err(e) => state.status = format!("Error saving to file: {}", e),
    //     }
    // }
    // ui.same_line();
    // if ui.button("Load from File") {
    //     match state
    //         .editor_context
    //         .load_current_editor_state_from_file("save_load_state.ini")
    //     {
    //         Ok(_) => {
    //             state.last_selected_nodes.clear();
    //             state.last_selected_links.clear();
    //             state.status =
    //                 "Loaded imnodes state from file. App state assumed to match.".to_string();
    //         }
    //         Err(e) => state.status = format!("Error loading from file: {}", e),
    //     }
    // }

    ui.separator();

    let current_panning = state.editor_context.get_panning();
    ui.text(format!(
        "Current Pan: {:.2}, {:.2}",
        current_panning.x, current_panning.y
    ));
    ui.same_line();
    if ui.button("Reset Pan") {
        state
            .editor_context
            .reset_panning(imnodes::ImVec2 { x: 0.0, y: 0.0 });
        state.status = "Panning reset".to_string();
    }

    ui.text(format!(
        "Selected Nodes: {}",
        state.last_selected_nodes.len()
    ));
    if state.last_selected_nodes.len() == 1 {
        let node_id = state.last_selected_nodes[0];
        // Check if the node still exists in our app state before getting position
        if state.nodes.iter().any(|n| n.id == node_id) {
            let screen_pos = node_id.get_position(CoordinateSystem::ScreenSpace);
            let editor_pos = node_id.get_position(CoordinateSystem::EditorSpace);
            let grid_pos = node_id.get_position(CoordinateSystem::GridSpace);
            ui.text(format!("  Node {node_id:?} Pos:"));
            ui.text(format!(
                "    Screen: {:.1}, {:.1}",
                screen_pos.x, screen_pos.y
            ));
            ui.text(format!(
                "    Editor: {:.1}, {:.1}",
                editor_pos.x, editor_pos.y
            ));
            ui.text(format!("    Grid:   {:.1}, {:.1}", grid_pos.x, grid_pos.y));

            ui.same_line();
            if ui.button("Deselect Node") {
                let _ = node_id.deselect();
            }
            ui.same_line();
            if ui.button("Snap to Grid") {
                let _ = node_id.snap_to_grid();
            }
        } else {
            // Node was likely removed after selection but before redraw
            ui.text(format!("  Node {node_id:?} (removed)"));
        }
    } else if state.last_selected_nodes.len() > 1 {
        ui.same_line();
        if ui.button("Clear Node Selection") {
            state.editor_context.clear_node_selection();
            state.last_selected_nodes.clear();
        }
    }

    ui.text(format!(
        "Selected Links: {}",
        state.last_selected_links.len()
    ));
    if !state.last_selected_links.is_empty() {
        ui.same_line();
        if ui.button("Clear Link Selection") {
            state.editor_context.clear_link_selection();
            state.last_selected_links.clear();
        }
    }

    ui.separator();
    ui.text(format!("Status: {}", state.status));
    ui.separator();

    // Store context menu click position *outside* the editor closure
    let mut context_menu_pos = None;
    let outer_scope = editor(&mut state.editor_context, |mut editor_scope| {
        // Detect context menu click *inside* the editor scope
        if editor_scope.is_hovered()
            && (ui.is_key_released(imgui::Key::A) || ui.is_mouse_clicked(imgui::MouseButton::Right))
        {
            // Store the position where the node should be added
            context_menu_pos = Some(ui.io().mouse_pos);
        }

        // Iterate using indices to allow mutable borrow inside slider closure
        for i in 0..state.nodes.len() {
            // Need to get these before the mutable borrow below
            let node_id = state.nodes[i].id;
            let input_pin = state.nodes[i].input;
            let output_pin = state.nodes[i].output;
            let attr_id = state.id_gen.next_attribute(); // Regenerate attribute ID each frame

            editor_scope.add_node(node_id, |mut node_scope| {
                node_scope.add_titlebar(|| {
                    ui.text(format!("Node {node_id:?}"));
                });

                node_scope.add_input(input_pin, PinShape::CircleFilled, || {
                    ui.text("In");
                });

                ui.same_line();

                // Add a simple widget like in multi_editor
                node_scope.add_static_attribute(attr_id, || {
                    if let Some(_node_mut) = state.nodes.get_mut(i) {
                        ui.set_next_item_width(80.0);
                    }
                });

                ui.same_line();

                node_scope.add_output(output_pin, PinShape::CircleFilled, || {
                    ui.text("Out");
                });
            });
        }

        for link_data in &state.links {
            editor_scope.add_link(link_data.id, link_data.end_pin, link_data.start_pin);
        }
    });

    // Handle node addition request *after* the editor scope ends
    if let Some(pos) = context_menu_pos {
        state.add_node(Some(pos));
        // state.add_node_at = None; // Not needed anymore
    } else if let Some(pos) = state.add_node_at {
        if pos[0] == -1.0 {
            // Check for sentinel value from button click
            state.add_node(None);
        }
        state.add_node_at = None; // Reset request
    }

    // Update stored selections for the *next* frame
    state.last_selected_nodes = outer_scope.selected_nodes();
    state.last_selected_links = outer_scope.selected_links();

    if let Some(new_link) = outer_scope.links_created() {
        let new_app_link = AppLink {
            id: state.id_gen.next_link(),
            start_pin: new_link.start_pin,
            end_pin: new_link.end_pin,
        };
        if !state
            .links
            .iter()
            .any(|l| l.start_pin == new_app_link.start_pin && l.end_pin == new_app_link.end_pin)
        {
            state.links.push(new_app_link);
            state.status = format!(
                "Created Link from pin {:?} to {:?}",
                new_link.start_pin, new_link.end_pin
            );
        }
    }

    if let Some(destroyed_link_id) = outer_scope.get_destroyed_link() {
        let initial_len = state.links.len();
        state.links.retain(|link| link.id != destroyed_link_id);
        if state.links.len() < initial_len {
            state.status = format!("Removed Link {destroyed_link_id:?}");
        }
    }

    // Pop the editor's unique ID
    editor_id.pop();
}

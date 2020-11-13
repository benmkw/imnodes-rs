#![deny(missing_docs)]

/*!
Bindings for [imnodes](https://github.com/Nelarius/imnodes) using [cimnodes](https://github.com/cimgui/cimnodes)
*/

use imnodes_sys as sys;

/// export all low level functions
#[cfg(feature = "include_low_level_bindings")]
pub mod internal {
    pub use imnodes_sys::*;
}

mod context;
pub use context::*;

mod helpers;
pub use helpers::*;

mod styling;
pub use styling::*;

mod scopes;
pub use scopes::*;

// maybe wrap those (same decision as in implot-rs)
pub use sys::{ImVec2, Style};

/// used to generate unique identifers for elements
pub struct IdentifierGenerator {
    current_node: i32,
    current_pin: i32,
    current_link: i32,
}

impl IdentifierGenerator {
    /// create
    pub(crate) fn new() -> Self {
        Self {
            current_node: 0,
            // input and output pins use the same pool, they must not overlap,
            // attributes as well as far as I can see
            current_pin: 0,
            current_link: 0,
        }
    }

    /// Id for a Node
    pub fn next_node(&mut self) -> NodeId {
        let id = self.current_node;
        self.current_node += 1;
        NodeId { id }
    }

    /// Id for an input pin
    pub fn next_input_pin(&mut self) -> InputPinId {
        let id = self.current_pin;
        self.current_pin += 1;
        InputPinId { id }
    }

    /// Id for an output pin
    pub fn next_output_pin(&mut self) -> OutputPinId {
        let id = self.current_pin;
        self.current_pin += 1;
        OutputPinId { id }
    }

    /// Id for an attribute in a Node
    pub fn next_attribute(&mut self) -> AttributeId {
        let id = self.current_pin;
        self.current_pin += 1;
        AttributeId { id }
    }

    /// Id for a link
    pub fn next_link(&mut self) -> LinkId {
        let id = self.current_link;
        self.current_link += 1;
        LinkId { id }
    }
}

/// Identifier for Attributes in nodes
/// TODO what precise uniqueness constraints do these have?
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct AttributeId {
    id: i32,
}

impl Into<i32> for AttributeId {
    fn into(self) -> i32 {
        self.id
    }
}

/// TODO look this up
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum CoordinateSystem {
    /// TODO
    ScreenSpace,
    /// TODO
    EditorSpace,
    /// TODO
    GridSpace,
}

/// Identifier for a Node
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct NodeId {
    id: i32,
}

impl NodeId {
    /// can the node be moved with the mouse
    pub fn set_draggable(&self, draggable: bool) {
        unsafe { sys::imnodes_SetNodeDraggable(self.id, draggable) };
    }

    /// EditorContextMoveToNode
    pub fn move_editor_to(&self) {
        unsafe { sys::imnodes_EditorContextMoveToNode(self.id) };
    }

    /// get the size of the node
    pub fn get_dimensions(&self) -> ImVec2 {
        let mut dimension = ImVec2 { x: 0.0, y: 0.0 };
        unsafe { sys::imnodes_GetNodeDimensions(&mut dimension as _, self.id) };
        dimension
    }

    /// move the node
    pub fn set_position(&self, x: f32, y: f32, coordinate_sytem: CoordinateSystem) {
        let pos = ImVec2 { x, y };
        match coordinate_sytem {
            CoordinateSystem::ScreenSpace => unsafe {
                sys::imnodes_SetNodeScreenSpacePos(self.id, pos)
            },
            CoordinateSystem::EditorSpace => unsafe {
                sys::imnodes_SetNodeEditorSpacePos(self.id, pos)
            },
            CoordinateSystem::GridSpace => unsafe {
                sys::imnodes_SetNodeGridSpacePos(self.id, pos)
            },
        };
    }

    /// get the coordinated of the node
    pub fn get_position(&self, coordinate_sytem: CoordinateSystem) -> ImVec2 {
        let mut pos = ImVec2 { x: 0.0, y: 0.0 };

        match coordinate_sytem {
            CoordinateSystem::ScreenSpace => unsafe {
                sys::imnodes_GetNodeScreenSpacePos(&mut pos as _, self.id)
            },
            CoordinateSystem::EditorSpace => unsafe {
                sys::imnodes_GetNodeEditorSpacePos(&mut pos as _, self.id)
            },
            CoordinateSystem::GridSpace => unsafe {
                sys::imnodes_GetNodeGridSpacePos(&mut pos as _, self.id)
            },
        };

        pos
    }
}

impl Into<i32> for NodeId {
    fn into(self) -> i32 {
        self.id
    }
}

/// either input or output pin
/// like attribute_id in the original source
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct PinId {
    id: i32,
}

impl PinId {
    /// TODO test
    pub fn is_start_of_link(&self, scope: &ScopeNone) -> bool {
        Some(*self) == scope.from_where_link_started()
    }

    /// TODO test
    pub fn dropped_link(&self, including_detached_links: bool, scope: &ScopeNone) -> bool {
        Some(*self) == scope.from_where_link_dropped(including_detached_links)
    }
}

/// Id for an input
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct InputPinId {
    id: i32,
}

impl Into<i32> for InputPinId {
    fn into(self) -> i32 {
        self.id
    }
}

impl Into<PinId> for InputPinId {
    fn into(self) -> PinId {
        PinId { id: self.id }
    }
}

/// Id for an output
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct OutputPinId {
    id: i32,
}

impl Into<i32> for OutputPinId {
    fn into(self) -> i32 {
        self.id
    }
}

impl Into<PinId> for OutputPinId {
    fn into(self) -> PinId {
        PinId { id: self.id }
    }
}

/// Id for a link
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct LinkId {
    id: i32,
}

impl LinkId {
    /// IsLinkDestroyed
    /// checks if the link of this LinkId got removed
    pub fn is_removed(&self, scope: &ScopeNone) -> bool {
        Some(*self) == scope.get_dropped_link()
    }
}

impl Into<i32> for LinkId {
    fn into(self) -> i32 {
        self.id
    }
}

/// makes it possible to detect if the mouse if at the positoin of the element
pub trait Hoverable {
    /// dispatches to one of the following:
    /// isNodeHovered
    /// isPinHovered
    /// isLinkHovered
    ///
    /// there is also [is_editor_hovered()] which does not depend on the scope
    fn is_hovered(self, _: &ScopeNone) -> bool;
}

impl Hoverable for OutputPinId {
    /// isPinHovered
    fn is_hovered(self, scope: &ScopeNone) -> bool {
        Some(PinId { id: self.id }) == scope.get_hovered_pin()
    }
}

impl Hoverable for InputPinId {
    /// isPinHovered
    fn is_hovered(self, scope: &ScopeNone) -> bool {
        Some(PinId { id: self.id }) == scope.get_hovered_pin()
    }
}

impl Hoverable for NodeId {
    /// isNodeHovered
    fn is_hovered(self, _: &ScopeNone) -> bool {
        Some(self) == get_hovered_node()
    }
}

impl Hoverable for LinkId {
    /// isLinkHovered
    fn is_hovered(self, scope: &ScopeNone) -> bool {
        Some(self) == scope.get_hovered_link()
    }
}

/// IsNodeHovered
pub fn get_hovered_node() -> Option<NodeId> {
    let mut id: i32 = -1;
    let ok = unsafe { sys::imnodes_IsNodeHovered(&mut id as _) };
    if ok {
        Some(NodeId { id })
    } else {
        None
    }
}

#[allow(missing_docs)]
/// the cpp code makes sure to put the input and output types in the right fields
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Link {
    pub start_node: NodeId,
    pub end_node: NodeId,
    pub start_pin: OutputPinId,
    pub end_pin: InputPinId,
    pub craeated_from_snap: bool,
}

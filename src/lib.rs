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

mod settings;
pub use settings::*;

// maybe wrap those (same decision as in implot-rs)
pub use sys::{EditorContext, ImVec2, Style};

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

    /// Id for a link
    pub fn next_link(&mut self) -> LinkId {
        let id = self.current_link;
        self.current_link += 1;
        LinkId { id }
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
    pub fn dimensions(&self) -> ImVec2 {
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
    pub fn position(&self, coordinate_sytem: CoordinateSystem) -> ImVec2 {
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
        Some(*self) == scope.is_link_started()
    }

    /// TODO test
    pub fn dropped_link(&self, including_detached_links: bool, scope: &ScopeNone) -> bool {
        Some(*self) == scope.is_link_dropped(including_detached_links)
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
    pub fn removed(&self, scope: &ScopeNone) -> bool {
        Some(*self) == scope.link_removed()
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
    /// IsEditorHovered
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

impl Hoverable for ScopeEditor {
    /// IsEditorHovered
    fn is_hovered(self, _: &ScopeNone) -> bool {
        is_editor_hovered()
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

/// BeginNodeEditor
/// ...
/// EndNodeEditor
pub fn editor<F: FnOnce(ScopeEditor)>(_scope: &Context, f: F) -> ScopeNone {
    unsafe { sys::imnodes_BeginNodeEditor() };
    f(ScopeEditor {});
    unsafe { sys::imnodes_EndNodeEditor() };
    ScopeNone {}
}

/// Original Scopes turned into compile time checks:
/// Scope_None = 1,
#[derive(Debug)]
pub struct ScopeNone {}
impl ScopeNone {
    /// check the position of the mosue
    pub fn is_hovered(&self, id: impl Hoverable) -> bool {
        id.is_hovered(&self)
    }

    /// IsLinkStarted
    pub fn link_started_at<T: Into<PinId>>(&self, id: T) -> bool {
        let id: PinId = id.into();
        id.is_start_of_link(self)
    }

    /// IsLinkDropped
    pub fn link_dropped_from<T: Into<PinId>>(&self, id: T, including_detached_links: bool) -> bool {
        let id: PinId = id.into();
        id.dropped_link(including_detached_links, self)
    }

    /// NumSelectedNodes
    /// selected_nodes builds on top of this
    pub fn num_selected_nodes(&self) -> u32 {
        let num = unsafe { sys::imnodes_NumSelectedNodes() };
        assert!(num > 0);
        num as u32
    }

    /// NumSelectedLinks
    /// selected_links builds on top of this
    pub fn num_selected_links(&self) -> u32 {
        let num = unsafe { sys::imnodes_NumSelectedLinks() };
        assert!(num > 0);
        num as u32
    }

    /// GetSelectedNodes
    pub fn selected_nodes(&self) -> Vec<NodeId> {
        let nr_nodes = self.num_selected_nodes() as usize;
        let mut nodes = vec![NodeId { id: 0 }; nr_nodes];
        unsafe { sys::imnodes_GetSelectedNodes(nodes.as_mut_ptr() as _) };
        nodes
    }

    /// GetSelectedLinks
    pub fn selected_links(&self) -> Vec<LinkId> {
        let nr_links = self.num_selected_links() as usize;
        let mut links = vec![LinkId { id: 0 }; nr_links];
        unsafe { sys::imnodes_GetSelectedLinks(links.as_mut_ptr() as _) };
        links
    }

    /// IsLinkCreated
    pub fn links_created(&self) -> Option<Link> {
        let mut started_at_node_id: i32 = -1;
        let mut started_at_attribute_id: i32 = -1;
        let mut ended_at_node_id: i32 = -1;
        let mut ended_at_attribute_id: i32 = -1;
        let mut created_from_snap: bool = true;

        let is_created = unsafe {
            sys::imnodes_IsLinkCreatedIntPtr(
                &mut started_at_node_id as _,
                &mut started_at_attribute_id as _,
                &mut ended_at_node_id as _,
                &mut ended_at_attribute_id as _,
                &mut created_from_snap as *mut bool,
            )
        };

        // let is_created = unsafe {
        //     sys::imnodes_IsLinkCreatedBoolPtr(
        //         &mut started_at_attribute_id as _,
        //         &mut ended_at_attribute_id as _,
        //         &mut created_from_snap as *mut bool,
        //     )
        // };

        if is_created {
            Some(Link {
                start_node: NodeId {
                    id: started_at_node_id,
                },
                end_node: NodeId {
                    id: ended_at_node_id,
                },
                start_pin: OutputPinId {
                    id: started_at_attribute_id,
                },
                end_pin: InputPinId {
                    id: ended_at_attribute_id,
                },
                craeated_from_snap: created_from_snap,
            })
        } else {
            None
        }
    }

    /// IsLinkDestroyed
    /// ... with a bit less drastic naming :D
    pub fn link_removed(&self) -> Option<LinkId> {
        let mut id: i32 = -1;
        if unsafe { sys::imnodes_IsLinkDestroyed(&mut id as _) } {
            Some(LinkId { id })
        } else {
            None
        }
    }

    /// IsPinHovered
    pub fn get_hovered_pin(&self) -> Option<PinId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsPinHovered(&mut id as _) };
        if ok {
            Some(PinId { id })
        } else {
            None
        }
    }

    /// IsLinkHovered
    pub fn get_hovered_link(&self) -> Option<LinkId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsLinkHovered(&mut id as _) };
        if ok {
            Some(LinkId { id })
        } else {
            None
        }
    }

    /// IsLinkStarted
    pub fn is_link_started(&self) -> Option<PinId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsLinkStarted(&mut id as _) };
        if ok {
            Some(PinId { id })
        } else {
            None
        }
    }

    /// IsLinkDropped
    pub fn is_link_dropped(&self, including_detached_links: bool) -> Option<PinId> {
        let mut id: i32 = -1;
        let ok = unsafe { sys::imnodes_IsLinkDropped(&mut id as _, including_detached_links) };
        if ok {
            Some(PinId { id })
        } else {
            None
        }
    }
}

/// cpp makes sure to put the input and output types in the right fields
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Link {
    start_node: NodeId,
    end_node: NodeId,
    start_pin: OutputPinId,
    end_pin: InputPinId,
    craeated_from_snap: bool,
}

/// Scope_Editor = 1 << 1,
#[derive(Debug)]
pub struct ScopeEditor {}
impl ScopeEditor {
    /// BeginNode
    /// ...
    /// EndNode
    pub fn node<F: FnOnce(ScopeNode)>(&self, id: NodeId, f: F) {
        unsafe { sys::imnodes_BeginNode(id.into()) }

        f(ScopeNode {});
        unsafe { sys::imnodes_EndNode() };
    }

    /// Link
    pub fn link(&self, id: LinkId, input: InputPinId, output: OutputPinId) {
        unsafe { sys::imnodes_Link(id.into(), input.into(), output.into()) }
    }
}

/// Scope_Node = 1 << 2,
#[derive(Debug)]
pub struct ScopeNode {}
impl ScopeNode {
    /// BeginNodeTitleBar
    /// ....
    /// EndNodeTitleBar
    pub fn titlebar<F: FnOnce()>(&self, f: F) {
        unsafe { sys::imnodes_BeginNodeTitleBar() }
        f();
        unsafe { sys::imnodes_EndNodeTitleBar() }
    }

    /// BeginInputAttribute
    /// ...
    /// EndInputAttribute
    pub fn input<F: FnOnce()>(&self, id: InputPinId, shape: PinShape, f: F) {
        unsafe { sys::imnodes_BeginInputAttribute(id.into(), shape as i32) };
        f();
        unsafe { sys::imnodes_EndInputAttribute() };
    }

    /// BeginOutputAttribute
    /// ...
    /// EndOutputAttribute
    pub fn output<F: FnOnce()>(&self, id: OutputPinId, shape: PinShape, f: F) {
        unsafe { sys::imnodes_BeginOutputAttribute(id.into(), shape as i32) };
        f();
        unsafe { sys::imnodes_EndOutputAttribute() };
    }
}

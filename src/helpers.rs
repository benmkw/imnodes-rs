use crate::EditorContext;
use crate::sys;

/// Provides helper methods for the [`EditorContext`].
impl EditorContext {
    /// Gets the current panning offset of the editor canvas.
    #[doc(alias = "EditorContextGetPanning")]
    #[must_use]
    pub fn get_panning(&self) -> sys::ImVec2 {
        let mut position = sys::ImVec2 { x: 0.0, y: 0.0 };
        // Safety: C API call. `position` is written to by the function.
        // Assumes the current context is set correctly before calling editor functions.
        unsafe {
            sys::imnodes_EditorContextGetPanning(core::ptr::from_mut(&mut position));
        }
        position
    }

    /// Sets the panning offset of the editor canvas.
    #[doc(alias = "EditorContextResetPanning")]
    pub fn reset_panning(&self, pos: sys::ImVec2) {
        // Safety: C API call. Assumes `pos` is valid.
        // Assumes the current context is set correctly.
        unsafe { sys::imnodes_EditorContextResetPanning(pos) };
    }

    /// Clears the current selection of nodes.
    /// If a specific node ID is provided via [`crate::NodeId::deselect`], only that node is deselected.
    #[doc(alias = "ClearNodeSelection")]
    pub fn clear_node_selection(&self) {
        // Calls the _Nil version which clears all node selections.
        // Safety: C API call. Assumes the current context is set correctly.
        unsafe { sys::imnodes_ClearNodeSelection_Nil() };
    }

    /// Clears the current selection of links.
    /// If a specific link ID is provided via [`crate::LinkId::deselect`], only that link is deselected.
    #[doc(alias = "ClearLinkSelection")]
    pub fn clear_link_selection(&self) {
        // Calls the _Nil version which clears all link selections.
        // Safety: C API call. Assumes the current context is set correctly.
        unsafe { sys::imnodes_ClearLinkSelection_Nil() };
    }
}

/// Checks if the last created attribute (input, output, or static) is currently active.
///
/// An attribute is active if its UI content is being interacted with (e.g., dragging a slider).
/// This must be called immediately after the attribute's content has been rendered within
/// its corresponding `add_input`/`add_output`/`add_static_attribute` block.
#[doc(alias = "IsAttributeActive")]
#[must_use]
pub fn is_last_attribute_active() -> bool {
    // Safety: C API call. Relies on immediate-mode state.
    unsafe { sys::imnodes_IsAttributeActive() }
}

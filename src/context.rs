use crate::sys;
use crate::{StyleDesc, StyleVar};
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::path::Path;

/// High-level wrapper for `imnodes_sys::ImNodesIO`.
///
/// Use `EditorContext::io()` to access this struct.
#[derive(Debug)]
#[repr(transparent)]
pub struct Io(sys::ImNodesIO);

impl Io {
    /// Gets a reference to the settings for controlling multiple selection with a modifier key.
    ///
    /// # Safety
    /// The returned `MultipleSelectModifier` contains a raw pointer (`*const bool`).
    /// This pointer is managed by ImGui, and care must be taken to ensure its validity
    /// throughout its usage. Do not use after the parent `Io` struct might have been dropped
    /// or reallocated.
    pub fn multiple_select_modifier(&self) -> MultipleSelectModifier {
        MultipleSelectModifier {
            modifier: self.0.MultipleSelectModifier.Modifier,
        }
    }

    /// Sets the modifier key for multiple selection for the duration of the closure.
    pub fn with_multiple_select_modifier<F: FnOnce(&mut Self)>(&mut self, modifier: &bool, f: F) {
        let old_modifier = self.0.MultipleSelectModifier.Modifier;
        self.0.MultipleSelectModifier.Modifier = modifier as *const bool;
        f(self);
        self.0.MultipleSelectModifier.Modifier = old_modifier;
    }

    /// Gets a reference to the settings for emulating a three-button mouse.
    pub fn emulate_three_button_mouse(&self) -> EmulateThreeButtonMouse {
        EmulateThreeButtonMouse {
            modifier: self.0.EmulateThreeButtonMouse.Modifier,
        }
    }

    /// Sets the modifier key for emulating a three-button mouse for the duration of the closure.
    pub fn with_emulate_three_button_mouse<F: FnOnce(&mut Self)>(&mut self, modifier: &bool, f: F) {
        let old_modifier = self.0.EmulateThreeButtonMouse.Modifier;
        self.0.EmulateThreeButtonMouse.Modifier = modifier as *const bool;
        f(self);
        self.0.EmulateThreeButtonMouse.Modifier = old_modifier;
    }

    /// Gets a reference to the settings for detaching links with a modifier click.
    pub fn link_detach_with_modifier_click(&self) -> LinkDetachWithModifierClick {
        LinkDetachWithModifierClick {
            modifier: self.0.LinkDetachWithModifierClick.Modifier,
        }
    }

    /// Sets the modifier key for detaching links with a click for the duration of the closure.
    pub fn with_link_detach_with_modifier_click<F: FnOnce(&mut Self)>(
        &mut self,
        modifier: &bool,
        f: F,
    ) {
        let old_modifier = self.0.LinkDetachWithModifierClick.Modifier;
        self.0.LinkDetachWithModifierClick.Modifier = modifier as *const bool;
        f(self);
        self.0.LinkDetachWithModifierClick.Modifier = old_modifier;
    }

    /// Sets the mouse button used for panning the editor (by dragging in the background).
    /// Default is usually Middle button.
    /// Pass `None` to disable specific button panning (though ImNodes behavior for -1 is implementation defined).
    pub fn set_alt_mouse_button(&mut self, button: Option<imgui::MouseButton>) {
        self.0.AltMouseButton = button.map(|b| b as i32).unwrap_or(-1);
    }

    /// Gets the auto-panning speed (when dragging near edges).
    pub fn auto_panning_speed(&self) -> f32 {
        self.0.AutoPanningSpeed
    }

    /// Sets the auto-panning speed.
    pub fn set_auto_panning_speed(&mut self, speed: f32) {
        self.0.AutoPanningSpeed = speed;
    }
}

/// A safe wrapper for `imnodes_sys::MultipleSelectModifier`.
#[derive(Debug, Copy, Clone)]
pub struct MultipleSelectModifier {
    modifier: *const bool,
}

impl MultipleSelectModifier {
    /// Returns the currently set modifier key.
    pub fn get(&self) -> bool {
        // Safety: The pointer is assumed to be valid as long as the parent Io is valid.
        unsafe { *self.modifier }
    }
}

/// A safe wrapper for `imnodes_sys::EmulateThreeButtonMouse`.
#[derive(Debug, Copy, Clone)]
pub struct EmulateThreeButtonMouse {
    modifier: *const bool,
}

impl EmulateThreeButtonMouse {
    pub fn get(&self) -> bool {
        unsafe { *self.modifier }
    }
}

/// A safe wrapper for `imnodes_sys::LinkDetachWithModifierClick`.
#[derive(Debug, Copy, Clone)]
pub struct LinkDetachWithModifierClick {
    modifier: *const bool,
}

impl LinkDetachWithModifierClick {
    pub fn get(&self) -> bool {
        unsafe { *self.modifier }
    }
}

/// An editor context corresponds to a set of nodes in a single workspace
///
/// By default, the library creates an editor context behind the scenes, so using any of the imnodes
/// functions doesn't require you to explicitly create a context. However, creating explicit contexts
/// allows for multiple editor instances.
#[derive(Debug)]
pub struct EditorContext<'a> {
    pub(crate) raw: *mut sys::ImNodesEditorContext,
    _phantom: PhantomData<&'a Context>,
}

impl<'a> EditorContext<'a> {
    /// Pushes multiple style modifications onto the stack for the duration of the closure.
    ///
    /// This ensures that the styles are applied to the editor context (e.g. before calling `editor()`).
    pub fn push_style<F: FnOnce(&Self)>(&self, desc: &StyleDesc<'_>, f: F) {
        // Ensure this context is set before pushing styles
        let _ = self.set_as_current_editor();

        unsafe {
            for flag in desc.flags {
                sys::imnodes_PushAttributeFlag(*flag as i32);
            }
            for var in desc.vars {
                match var {
                    StyleVar::GridSpacing(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_GridSpacing as i32,
                        *v,
                    ),
                    StyleVar::NodeCornerRounding(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_NodeCornerRounding as i32,
                        *v,
                    ),
                    StyleVar::NodePadding(v) => sys::imnodes_PushStyleVar_Vec2(
                        sys::ImNodesStyleVar__ImNodesStyleVar_NodePadding as i32,
                        sys::ImVec2_c { x: v[0], y: v[1] },
                    ),
                    StyleVar::NodeBorderThickness(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_NodeBorderThickness as i32,
                        *v,
                    ),
                    StyleVar::LinkThickness(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_LinkThickness as i32,
                        *v,
                    ),
                    StyleVar::LinkLineSegmentsPerLength(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_LinkLineSegmentsPerLength as i32,
                        *v,
                    ),
                    StyleVar::LinkHoverDistance(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_LinkHoverDistance as i32,
                        *v,
                    ),
                    StyleVar::PinCircleRadius(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinCircleRadius as i32,
                        *v,
                    ),
                    StyleVar::PinQuadSideLength(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinQuadSideLength as i32,
                        *v,
                    ),
                    StyleVar::PinTriangleSideLength(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinTriangleSideLength as i32,
                        *v,
                    ),
                    StyleVar::PinLineThickness(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinLineThickness as i32,
                        *v,
                    ),
                    StyleVar::PinHoverRadius(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinHoverRadius as i32,
                        *v,
                    ),
                    StyleVar::PinOffset(v) => sys::imnodes_PushStyleVar_Float(
                        sys::ImNodesStyleVar__ImNodesStyleVar_PinOffset as i32,
                        *v,
                    ),
                    StyleVar::MiniMapPadding(v) => sys::imnodes_PushStyleVar_Vec2(
                        sys::ImNodesStyleVar__ImNodesStyleVar_MiniMapPadding as i32,
                        sys::ImVec2_c { x: v[0], y: v[1] },
                    ),
                    StyleVar::MiniMapOffset(v) => sys::imnodes_PushStyleVar_Vec2(
                        sys::ImNodesStyleVar__ImNodesStyleVar_MiniMapOffset as i32,
                        sys::ImVec2_c { x: v[0], y: v[1] },
                    ),
                }
            }
            for (style, color) in desc.colors {
                let c: imgui::ImColor32 = *color;
                sys::imnodes_PushColorStyle(*style as i32, c.into());
            }
        }

        f(self);

        unsafe {
            for _ in 0..desc.colors.len() {
                sys::imnodes_PopColorStyle();
            }
            if !desc.vars.is_empty() {
                sys::imnodes_PopStyleVar(desc.vars.len() as i32);
            }
            for _ in 0..desc.flags.len() {
                sys::imnodes_PopAttributeFlag();
            }
        }
    }

    /// Associates this editor context with the current ImGui window.
    ///
    /// This should be called before [`crate::editor()`].
    #[doc(alias = "EditorContextSet")]
    #[must_use]
    pub fn set_as_current_editor(&self) -> &Self {
        // Safety: C API call. Sets the thread-local current editor context.
        unsafe { sys::imnodes_EditorContextSet(self.raw) };
        self
    }

    /// Creates a new identifier generator associated with this editor context.
    ///
    /// Each editor should ideally use its own generator to avoid ID clashes
    /// if multiple editors are used in the same application.
    #[must_use]
    pub fn new_identifier_generator(&self) -> crate::IdentifierGenerator {
        // The generator itself is context-agnostic, but creating it via the context
        // encourages correct usage patterns.
        crate::IdentifierGenerator::new()
    }

    /// Saves the state of the currently active editor context to a string.
    ///
    /// Returns `None` if saving fails or the resulting string is invalid UTF-8.
    #[doc(alias = "SaveCurrentEditorStateToIniString")]
    #[must_use]
    pub fn save_current_editor_state_to_string(&self) -> Option<String> {
        // Ensure this context is set before saving 'current' state
        let _ = self.set_as_current_editor();
        let mut data_size: usize = 0;
        // Safety: C API call. `data_size` is written to by the function.
        // The returned pointer points to memory managed by imnodes (GImNodes->TextBuffer).
        let char_ptr =
            unsafe { sys::imnodes_SaveCurrentEditorStateToIniString(&mut data_size as *mut _) };
        if char_ptr.is_null() || data_size == 0 {
            return None;
        }
        // Safety: imnodes returns a valid null-terminated C string.
        // We use CStr::from_ptr to wrap it safely.
        unsafe { CStr::from_ptr(char_ptr) }
            .to_str()
            .ok()
            .map(String::from)
    }

    /// Saves the state of *this specific* editor context to a string.
    ///
    /// Returns `None` if saving fails or the resulting string is invalid UTF-8.
    #[doc(alias = "SaveEditorStateToIniString")]
    #[must_use]
    pub fn save_editor_state_to_string(&self) -> Option<String> {
        let mut data_size: usize = 0;
        // Safety: C API call. `data_size` is written to by the function.
        // The returned pointer points to memory managed by imnodes.
        let char_ptr =
            unsafe { sys::imnodes_SaveEditorStateToIniString(self.raw, &mut data_size as *mut _) };
        if char_ptr.is_null() || data_size == 0 {
            return None;
        }
        // Safety: imnodes returns a valid null-terminated C string.
        unsafe { CStr::from_ptr(char_ptr) }
            .to_str()
            .ok()
            .map(String::from)
    }

    /// Loads state into the currently active editor context from a string.
    #[doc(alias = "LoadCurrentEditorStateFromIniString")]
    pub fn load_current_editor_state_from_string(&self, data: &str) {
        // Ensure this context is set before loading 'current' state
        let _ = self.set_as_current_editor();
        // Safety: C API call. Assumes `data` points to valid memory for its length.
        unsafe {
            sys::imnodes_LoadCurrentEditorStateFromIniString(data.as_ptr().cast(), data.len())
        }
    }

    /// Loads state into *this specific* editor context from a string.
    #[doc(alias = "LoadEditorStateFromIniString")]
    pub fn load_editor_state_from_string(&self, data: &str) {
        // Safety: C API call. Assumes `data` points to valid memory for its length.
        unsafe {
            sys::imnodes_LoadEditorStateFromIniString(self.raw, data.as_ptr().cast(), data.len())
        }
    }

    /// Saves the state of the currently active editor context to an INI file.
    #[doc(alias = "SaveCurrentEditorStateToIniFile")]
    pub fn save_current_editor_state_to_file<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> std::io::Result<()> {
        // Ensure this context is set before saving 'current' state
        let _ = self.set_as_current_editor();
        let path_str = file_path.as_ref().to_string_lossy();
        // Use CString for null termination guarantee
        let c_path = CString::new(path_str.as_bytes())?;
        // Safety: C API call with a valid CString pointer.
        unsafe { sys::imnodes_SaveCurrentEditorStateToIniFile(c_path.as_ptr()) };
        Ok(())
    }

    /// Saves the state of *this specific* editor context to an INI file.
    #[doc(alias = "SaveEditorStateToIniFile")]
    pub fn save_editor_state_to_file<P: AsRef<Path>>(&self, file_path: P) -> std::io::Result<()> {
        let path_str = file_path.as_ref().to_string_lossy();
        let c_path = CString::new(path_str.as_bytes())?;
        // Safety: C API call with a valid CString pointer.
        unsafe { sys::imnodes_SaveEditorStateToIniFile(self.raw, c_path.as_ptr()) };
        Ok(())
    }

    /// Loads state into the currently active editor context from an INI file.
    #[doc(alias = "LoadCurrentEditorStateFromIniFile")]
    pub fn load_current_editor_state_from_file<P: AsRef<Path>>(
        &self,
        file_path: P,
    ) -> std::io::Result<()> {
        // Ensure this context is set before loading 'current' state
        let _ = self.set_as_current_editor();
        let path_str = file_path.as_ref().to_string_lossy();
        let c_path = CString::new(path_str.as_bytes())?;
        // Safety: C API call with a valid CString pointer.
        unsafe { sys::imnodes_LoadCurrentEditorStateFromIniFile(c_path.as_ptr()) };
        Ok(())
    }

    /// Loads state into *this specific* editor context from an INI file.
    #[doc(alias = "LoadEditorStateFromIniFile")]
    pub fn load_editor_state_from_file<P: AsRef<Path>>(&self, file_path: P) -> std::io::Result<()> {
        let path_str = file_path.as_ref().to_string_lossy();
        let c_path = CString::new(path_str.as_bytes())?;
        // Safety: C API call with a valid CString pointer.
        unsafe { sys::imnodes_LoadEditorStateFromIniFile(self.raw, c_path.as_ptr()) };
        Ok(())
    }

    /// Gets the current panning offset of the editor canvas.
    #[doc(alias = "EditorContextGetPanning")]
    #[must_use]
    pub fn get_panning(&self) -> sys::ImVec2_c {
        // Ensure this context is set before querying
        let _ = self.set_as_current_editor();
        // Safety: C API call that returns ImVec2_c directly.
        unsafe { sys::imnodes_EditorContextGetPanning() }
    }

    /// Sets the panning offset of the editor canvas.
    #[doc(alias = "EditorContextResetPanning")]
    pub fn reset_panning(&self, pos: impl Into<[f32; 2]>) {
        // Ensure this context is set before modifying
        let _ = self.set_as_current_editor();
        let [x, y] = pos.into();
        let pos = sys::ImVec2_c { x, y };
        // Safety: C API call. Assumes `pos` is valid.
        unsafe { sys::imnodes_EditorContextResetPanning(pos) };
    }

    /// Clears the current selection of nodes.
    /// If a specific node ID is provided via [`crate::NodeId::deselect`], only that node is deselected.
    #[doc(alias = "ClearNodeSelection")]
    pub fn clear_node_selection(&self) {
        // Ensure this context is set before modifying
        let _ = self.set_as_current_editor();
        // Calls the _Nil version which clears all node selections.
        // Safety: C API call.
        unsafe { sys::imnodes_ClearNodeSelection_Nil() };
    }

    /// Returns the number of currently selected nodes.
    #[doc(alias = "NumSelectedNodes")]
    #[must_use]
    pub fn num_selected_nodes(&self) -> u32 {
        let _ = self.set_as_current_editor();
        let num = unsafe { sys::imnodes_NumSelectedNodes() };
        num.max(0) as u32
    }

    /// Returns a vector containing the IDs of all currently selected nodes.
    #[doc(alias = "GetSelectedNodes")]
    #[must_use]
    pub fn get_selected_nodes(&self) -> Vec<crate::NodeId> {
        let nr_nodes = self.num_selected_nodes();
        if nr_nodes == 0 {
            return Vec::new();
        }
        let mut nodes: Vec<crate::NodeId> = Vec::with_capacity(nr_nodes as usize);
        unsafe {
            sys::imnodes_GetSelectedNodes(nodes.as_mut_ptr().cast::<i32>());
            nodes.set_len(nr_nodes as usize);
        }
        nodes
    }

    /// Returns the number of currently selected links.
    #[doc(alias = "NumSelectedLinks")]
    #[must_use]
    pub fn num_selected_links(&self) -> u32 {
        let _ = self.set_as_current_editor();
        let num = unsafe { sys::imnodes_NumSelectedLinks() };
        num.max(0) as u32
    }

    /// Returns a vector containing the IDs of all currently selected links.
    #[doc(alias = "GetSelectedLinks")]
    #[must_use]
    pub fn get_selected_links(&self) -> Vec<crate::LinkId> {
        let nr_links = self.num_selected_links();
        if nr_links == 0 {
            return Vec::new();
        }
        let mut links: Vec<crate::LinkId> = Vec::with_capacity(nr_links as usize);
        unsafe {
            sys::imnodes_GetSelectedLinks(links.as_mut_ptr().cast::<i32>());
            links.set_len(nr_links as usize);
        }
        links
    }

    /// Clears the current selection of links.
    /// If a specific link ID is provided via [`crate::LinkId::deselect`], only that link is deselected.
    #[doc(alias = "ClearLinkSelection")]
    pub fn clear_link_selection(&self) {
        // Ensure this context is set before modifying
        let _ = self.set_as_current_editor();
        // Calls the _Nil version which clears all link selections.
        // Safety: C API call.
        unsafe { sys::imnodes_ClearLinkSelection_Nil() };
    }
}

impl<'a> Drop for EditorContext<'a> {
    /// Frees the editor context if it was created explicitly via `Context::create_editor`.
    #[doc(alias = "EditorContextFree")]
    fn drop(&mut self) {
        // Safety: Frees the context created by `imnodes_EditorContextCreate`.
        // Only called if `owned` is true.
        unsafe {
            sys::imnodes_EditorContextFree(self.raw);
        }
    }
}

/// Represents the global imnodes context.
///
/// This should be created once at the start of the application, typically alongside
/// the `imgui::Context`.
#[doc(alias = "imnodes_CreateContext")]
#[derive(Debug)]
pub struct Context {
    // Renamed from ImnodesContext to avoid confusion
    context: *mut sys::ImNodesContext,
}

impl Default for Context {
    /// Creates a new global imnodes context. Equivalent to `Context::new()`.
    fn default() -> Self {
        Self::new(&ContextDesc::default())
    }
}

/// Descriptor for creating a new `Context`.
#[derive(Debug, Clone, Default)]
pub struct ContextDesc {}

/// Descriptor for creating a new `EditorContext`.
#[derive(Debug, Clone, Default)]
pub struct EditorContextDesc {}

impl Context {
    /// Creates a new global imnodes context.
    #[must_use]
    pub fn new(_desc: &ContextDesc) -> Self {
        // Safety: Creates the global imnodes context. Should be called once.
        let context = unsafe { sys::imnodes_CreateContext() };
        // Safety: Ensure the associated ImGui context is also set for imnodes.
        // We need to cast the pointer type because imgui-sys and imnodes-sys
        // define their own (but compatible) ImGuiContext types.
        unsafe {
            sys::imnodes_SetImGuiContext(imgui::sys::igGetCurrentContext() as *mut sys::ImGuiContext)
        };
        Self { context }
    }

    /// Creates an editor context for managing a single node editor workspace.
    ///
    /// This allows for multiple independent node editor instances.
    /// The returned context is tied to the lifetime of this `Context` to ensure safety.
    #[must_use]
    pub fn create_editor<'a>(&'a self, _desc: &EditorContextDesc) -> EditorContext<'a> {
        EditorContext {
            // Safety: Creates a new editor context associated with the global context.
            raw: unsafe { sys::imnodes_EditorContextCreate() },
            _phantom: PhantomData,
        }
    }
}

impl Drop for Context {
    /// Destroys the global imnodes context.
    fn drop(&mut self) {
        // Safety: Destroys the global context created by `imnodes_CreateContext`.
        unsafe { sys::imnodes_DestroyContext(self.context) }
    }
}

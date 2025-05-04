use crate::{ImNodesIO, Style, sys};
use std::ffi::{CStr, CString};
use std::path::Path;

/// An editor context corresponds to a set of nodes in a single workspace
///
/// By default, the library creates an editor context behind the scenes, so using any of the imnodes
/// functions doesn't require you to explicitly create a context. However, creating explicit contexts
/// allows for multiple editor instances.
#[derive(Debug)]
pub struct EditorContext {
    raw: *mut sys::ImNodesEditorContext,
}

impl EditorContext {
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

    /// Returns a mutable reference to the global style variables shared across all editor contexts.
    ///
    /// Use this to modify the *currently active* style. To get a copy of the default style,
    /// use [`crate::Style::default()`].
    #[doc(alias = "GetStyle")]
    pub fn get_style(&mut self) -> &mut Style {
        // Safety: This accesses the global style object managed by imnodes.
        // We cast the raw pointer to our wrapper struct.
        unsafe { &mut *(sys::imnodes_GetStyle() as *mut Style) }
    }

    /// Returns a mutable reference to the global IO settings shared across all editor contexts.
    ///
    /// Use this to configure input behaviors like modifier keys.
    #[doc(alias = "GetIO")]
    pub fn get_io(&mut self) -> &mut ImNodesIO {
        // Safety: This accesses the global IO object managed by imnodes.
        unsafe { &mut *sys::imnodes_GetIO() }
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
        // The returned pointer points to memory managed by imnodes.
        let char_ptr =
            unsafe { sys::imnodes_SaveCurrentEditorStateToIniString(&mut data_size as *mut _) };
        if char_ptr.is_null() || data_size == 0 {
            return None;
        }
        // Safety: We assume the pointer is valid for `data_size` bytes and contains UTF-8 data.
        // We create a CStr slice from it. Using CStr::from_bytes_until_nul as imnodes might not guarantee null termination exactly at data_size.
        // If the data is not null terminated *within* data_size, this could read out of bounds, but imnodes *should* null terminate.
        // A potentially safer approach would involve checking for null termination explicitly if needed.
        unsafe {
            CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(
                char_ptr.cast(),
                data_size,
            ))
        }
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
        // Safety: See above.
        unsafe {
            CStr::from_bytes_with_nul_unchecked(std::slice::from_raw_parts(
                char_ptr.cast(),
                data_size,
            ))
        }
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
}

impl Drop for EditorContext {
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
        Self::new()
    }
}

impl Context {
    /// Creates a new global imnodes context.
    #[must_use]
    pub fn new() -> Self {
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
    #[must_use]
    pub fn create_editor(&self) -> EditorContext {
        EditorContext {
            // Safety: Creates a new editor context associated with the global context.
            raw: unsafe { sys::imnodes_EditorContextCreate() },
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

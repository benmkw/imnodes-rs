// TOOO look at https://github.com/4bb4/implot-rs/blob/master/src/context.rs and see if this approach makes
// sense here too

/// An editor context corresponds to a set of nodes in a single workspace
///
/// By default, the library creates an editor context behind the scenes, so using any of the imnodes
/// functions doesn't require you to explicitly create a context.
pub struct EditorContext {
    raw: *mut imnodes_sys::EditorContext,
}

impl EditorContext {
    /// use this context now
    #[doc(alias = "EditorContextSet")]
    pub fn set_as_current_editor(&self) -> &Self {
        unsafe { imnodes_sys::imnodes_EditorContextSet(self.raw) };
        self
    }

    /// generate Singleton IdentifierGenerator
    pub fn new_identifier_generator(&self) -> crate::IdentifierGenerator {
        crate::IdentifierGenerator::new()
    }

    /// GetStyle
    /// TODO see Style_destroy, make sure this does not leak
    /// Returns the global style struct. See the struct declaration for default values.
    #[doc(alias = "GetStyle")]
    pub fn get_style(&self) -> &mut imnodes_sys::Style {
        unsafe { &mut *(imnodes_sys::imnodes_GetStyle() as *mut imnodes_sys::Style) }
    }
}

impl Drop for EditorContext {
    #[doc(alias = "EditorContextFree")]
    fn drop(&mut self) {
        unsafe {
            imnodes_sys::imnodes_EditorContextFree(self.raw);
        }
    }
}

/// imnodes_CreateContext
#[doc(alias = "CreateContext")]
pub struct Context {
    context: *mut imnodes_sys::Context,
}

impl Context {
    /// create global context
    pub fn new() -> Self {
        let context = unsafe { imnodes_sys::imnodes_CreateContext() };

        Self { context }
    }

    /// created the context for one editor/ grid
    pub fn create_editor(&self) -> EditorContext {
        EditorContext {
            raw: unsafe { imnodes_sys::imnodes_EditorContextCreate() },
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { imnodes_sys::imnodes_DestroyContext(self.context) }
    }
}

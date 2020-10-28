// TOOO look at https://github.com/4bb4/implot-rs/blob/master/src/context.rs and see if this approach makes
// sense here too

use imnodes_sys as sys;

/// needs to be unique for each editor
pub struct EditorContext {
    raw: *mut sys::EditorContext,
}

impl EditorContext {
    /// use this context now
    pub fn set_as_current_editor(&self) {
        unsafe { sys::imnodes_EditorContextSet(self.raw) };
    }

    /// generate Singleton IdentifierGenerator
    pub fn new_identifier_generator(&self) -> crate::IdentifierGenerator {
        crate::IdentifierGenerator::new()
    }

    /// GetStyle
    /// TODO see Steyle_destroy, make sure this does not leak
    pub fn get_style(&self) -> &mut sys::Style {
        unsafe { &mut *(sys::imnodes_GetStyle() as *mut sys::Style) }
    }
}

impl Drop for EditorContext {
    fn drop(&mut self) {
        unsafe {
            sys::imnodes_EditorContextFree(self.raw);
        }
    }
}

/// imnodes_Initialize
pub struct Context {}

impl Context {
    /// create global context
    pub fn new() -> Self {
        unsafe { sys::imnodes_Initialize() }

        Self {}
    }

    /// created the context for one editor/ grid
    pub fn create_editor(&self) -> EditorContext {
        EditorContext {
            raw: unsafe { sys::imnodes_EditorContextCreate() },
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { sys::imnodes_Shutdown() }
    }
}

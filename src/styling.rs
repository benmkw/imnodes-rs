#![allow(missing_docs)]

use crate::{sys, EditorContext};
use imgui::ImColor32;

impl EditorContext {
    /// dark color theme
    #[doc(alias = "StyleColorsDark")]
    pub fn set_style_colors_dark(&self) -> &Self {
        unsafe { sys::imnodes_StyleColorsDark() };
        self
    }

    /// classic color theme
    #[doc(alias = "StyleColorsClassic")]
    pub fn set_style_colors_classic(&self) -> &Self {
        unsafe { sys::imnodes_StyleColorsClassic() };
        self
    }

    /// light color theme
    #[doc(alias = "StyleColorsLight")]
    pub fn set_style_colors_light(&self) -> &Self {
        unsafe { sys::imnodes_StyleColorsLight() };
        self
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum ColorStyle {
    NodeBackground = sys::ColorStyle_ColorStyle_NodeBackground,
    NodeBackgroundHovered = sys::ColorStyle_ColorStyle_NodeBackgroundHovered,
    NodeBackgroundSelected = sys::ColorStyle_ColorStyle_NodeBackgroundSelected,
    NodeOutline = sys::ColorStyle_ColorStyle_NodeOutline,
    TitleBar = sys::ColorStyle_ColorStyle_TitleBar,
    TitleBarHovered = sys::ColorStyle_ColorStyle_TitleBarHovered,
    TitleBarSelected = sys::ColorStyle_ColorStyle_TitleBarSelected,
    Link = sys::ColorStyle_ColorStyle_Link,
    LinkHovered = sys::ColorStyle_ColorStyle_LinkHovered,
    LinkSelected = sys::ColorStyle_ColorStyle_LinkSelected,
    Pin = sys::ColorStyle_ColorStyle_Pin,
    PinHovered = sys::ColorStyle_ColorStyle_PinHovered,
    BoxSelector = sys::ColorStyle_ColorStyle_BoxSelector,
    BoxSelectorOutline = sys::ColorStyle_ColorStyle_BoxSelectorOutline,
    GridBackground = sys::ColorStyle_ColorStyle_GridBackground,
    GridLine = sys::ColorStyle_ColorStyle_GridLine,
    // Count = sys::ColorStyle_ColorStyle_Count,
}

impl ColorStyle {
    pub const COUNT: u32 = sys::ColorStyle_ColorStyle_Count;

    #[must_use = "need to call pop on ColorToken befor going out of scope"]
    #[doc(alias = "PushColorStyle")]
    pub fn push_color<C: Into<ImColor32>>(self, color: C, _: &EditorContext) -> ColorToken {
        let color: ImColor32 = color.into();
        unsafe { sys::imnodes_PushColorStyle(self as u32, color.into()) };
        ColorToken { ended: false }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ColorToken {
    ended: bool,
}

impl ColorToken {
    #[doc(alias = "PopColorStyle")]
    pub fn pop(mut self) {
        self.ended = true;
        unsafe { sys::imnodes_PopColorStyle() };
    }
}

// this could implicitly call pop/ pop but thats probably hiding bugs...
impl Drop for ColorToken {
    fn drop(&mut self) {
        if !self.ended {
            panic!("did not call pop on a color token");
        }
    }
}

/// The default size of each pin shape is balanced to occupy approximately the same surface area on the screen.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum StyleVar {
    GridSpacing = sys::StyleVar_StyleVar_GridSpacing,
    NodeCornerRounding = sys::StyleVar_StyleVar_NodeCornerRounding,
    NodePaddingHorizontal = sys::StyleVar_StyleVar_NodePaddingHorizontal,
    NodePaddingVertical = sys::StyleVar_StyleVar_NodePaddingVertical,
    NodeBorderThickness = sys::StyleVar_StyleVar_NodeBorderThickness,
    LinkThickness = sys::StyleVar_StyleVar_LinkThickness,
    LinkLineSegmentsPerLength = sys::StyleVar_StyleVar_LinkLineSegmentsPerLength,
    LinkHoverDistance = sys::StyleVar_StyleVar_LinkHoverDistance,
    /// The circle radius used when the pin shape is either [PinShape::Circle] or [PinShape::CircleFilled].
    PinCircleRadius = sys::StyleVar_StyleVar_PinCircleRadius,
    /// The quad side length used when the shape is either [PinShape::Quad] or [PinShape::QuadFilled].
    PinQuadSideLength = sys::StyleVar_StyleVar_PinQuadSideLength,
    /// The equilateral triangle side length used when the pin shape is either [PinShape::Triangle] or [PinShape::TriangleFilled].
    PinTriangleSideLength = sys::StyleVar_StyleVar_PinTriangleSideLength,
    /// The thickness of the line used when the pin shape is not filled.
    PinLineThickness = sys::StyleVar_StyleVar_PinLineThickness,
    /// The radius from the pin's center position inside of which it is detected as being hovered over.
    PinHoverRadius = sys::StyleVar_StyleVar_PinHoverRadius,
    /// Offsets the pins' positions from the edge of the node to the outside of the node.
    PinOffset = sys::StyleVar_StyleVar_PinOffset,
}

impl StyleVar {
    #[must_use = "need to call pop on StyleVarToken befor going out of scope"]
    #[doc(alias = "PushStyleVar")]
    pub fn push_val(self, value: f32, _: &EditorContext) -> StyleVarToken {
        unsafe { sys::imnodes_PushStyleVar(self as u32, value) };
        StyleVarToken { ended: false }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StyleVarToken {
    ended: bool,
}
impl StyleVarToken {
    #[doc(alias = "PopStyleVar")]
    pub fn pop(mut self) {
        self.ended = true;
        unsafe { sys::imnodes_PopStyleVar() };
    }
}

impl Drop for StyleVarToken {
    fn drop(&mut self) {
        if !self.ended {
            panic!("did not call pop on a style var token");
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum StyleFlag {
    // None = sys::StyleFlags_StyleFlags_None,
    NodeOutline = sys::StyleFlags_StyleFlags_NodeOutline,
    GridLines = sys::StyleFlags_StyleFlags_GridLines,
}

/// This enum controls the way attribute pins look.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum PinShape {
    Circle = sys::PinShape_PinShape_Circle,
    CircleFilled = sys::PinShape_PinShape_CircleFilled,
    Triangle = sys::PinShape_PinShape_Triangle,
    TriangleFilled = sys::PinShape_PinShape_TriangleFilled,
    Quad = sys::PinShape_PinShape_Quad,
    QuadFilled = sys::PinShape_PinShape_QuadFilled,
}

/// This enum controls the way the attribute pins behave.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AttributeFlag {
    None = sys::AttributeFlags_AttributeFlags_None,
    /// Allow detaching a link by left-clicking and dragging the link at a pin it is connected to.
    /// NOTE: the user has to actually delete the link for this to work. A deleted link can be
    /// detected by calling [crate::LinkId::is_removed()] after [crate::scopes::editor()].
    EnableLinkDetachWithDragClick =
        sys::AttributeFlags_AttributeFlags_EnableLinkDetachWithDragClick,
    /// Visual snapping of an in progress link will trigger IsLink Created/Destroyed events. Allows
    /// for previewing the creation of a link while dragging it across attributes. See here for demo:
    /// <https://github.com/Nelarius/imnodes/issues/41#issuecomment-647132113> NOTE: the user has to
    /// actually delete the link for this to work. A deleted link can be detected by calling
    /// [crate::LinkId::is_removed()] after [crate::scopes::editor()].
    EnableLinkCreationOnSnap = sys::AttributeFlags_AttributeFlags_EnableLinkCreationOnSnap,
}

impl EditorContext {
    /// Push a single AttributeFlags value. By default, only AttributeFlags_None is set.
    #[must_use = "need to call pop on AttributeFlagsToken befor going out of scope"]
    #[doc(alias = "PushAttributeFlag")]
    pub fn push(&self, flag: AttributeFlag) -> AttributeFlagToken {
        unsafe { sys::imnodes_PushAttributeFlag(flag as u32) };
        AttributeFlagToken { ended: false }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AttributeFlagToken {
    ended: bool,
}
impl AttributeFlagToken {
    #[doc(alias = "PopAttributeFlag")]
    pub fn pop(mut self) {
        self.ended = true;
        unsafe { sys::imnodes_PopAttributeFlag() };
    }
}

impl Drop for AttributeFlagToken {
    fn drop(&mut self) {
        if !self.ended {
            panic!("did not call pop on a style var token");
        }
    }
}

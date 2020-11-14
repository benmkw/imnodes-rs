#![allow(missing_docs)]

use crate::{sys, EditorContext};
use imgui::ImColor;

impl EditorContext {
    /// dark color theme
    pub fn set_style_colors_dark(&self) -> &Self {
        unsafe { sys::imnodes_StyleColorsDark() };
        self
    }

    /// classic color theme
    pub fn set_style_colors_classic(&self) -> &Self {
        unsafe { sys::imnodes_StyleColorsClassic() };
        self
    }

    /// light color theme
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
    pub fn push_color<C: Into<ImColor>>(self, color: C, _: &EditorContext) -> ColorToken {
        let color: ImColor = color.into();
        unsafe { sys::imnodes_PushColorStyle(self as u32, color.into()) };
        ColorToken { ended: false }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ColorToken {
    ended: bool,
}

impl ColorToken {
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
    PinCircleRadius = sys::StyleVar_StyleVar_PinCircleRadius,
    PinQuadSideLength = sys::StyleVar_StyleVar_PinQuadSideLength,
    PinTriangleSideLength = sys::StyleVar_StyleVar_PinTriangleSideLength,
    PinLineThickness = sys::StyleVar_StyleVar_PinLineThickness,
    PinHoverRadius = sys::StyleVar_StyleVar_PinHoverRadius,
    PinOffset = sys::StyleVar_StyleVar_PinOffset,
}

impl StyleVar {
    #[must_use = "need to call pop on StyleVarToken befor going out of scope"]
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AttributeFlag {
    None = sys::AttributeFlags_AttributeFlags_None,
    EnableLinkDetachWithDragClick =
        sys::AttributeFlags_AttributeFlags_EnableLinkDetachWithDragClick,
    EnableLinkCreationOnSnap = sys::AttributeFlags_AttributeFlags_EnableLinkCreationOnSnap,
}

impl EditorContext {
    #[must_use = "need to call pop on AttributeFlagsToken befor going out of scope"]
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

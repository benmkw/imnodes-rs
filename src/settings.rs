#![allow(missing_docs)]

use imgui::ImColor;

use crate::{sys, Context};

#[derive(Clone)]
#[repr(i32)]
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
    Count = sys::ColorStyle_ColorStyle_Count,
}

pub struct ColorToken {
    ended: bool,
}
impl ColorToken {
    pub fn end(mut self) {
        self.ended = true;
        unsafe { sys::imnodes_PopColorStyle() };
    }
}

// this could implicitly call end/ pop but thats probably hiding bugs...
impl Drop for ColorToken {
    fn drop(&mut self) {
        if !self.ended {
            panic!("did not call end on a color token");
        }
    }
}

#[must_use = "need to call end on ColorToken befor going out of scope"]
pub fn push_color_style<C: Into<ImColor>>(style: ColorStyle, color: C, _: &Context) -> ColorToken {
    let color: ImColor = color.into();
    unsafe { sys::imnodes_PushColorStyle(style as i32, color.into()) };
    ColorToken { ended: false }
}

#[derive(Clone)]
#[repr(i32)]
pub enum StyleVar {
    GridSpacing = sys::StyleVar_StyleVar_GridSpacing,
    NodeCornerRounding = sys::StyleVar_StyleVar_NodeCornerRounding,
    NodePaddingHorizontal = sys::StyleVar_StyleVar_NodePaddingHorizontal,
    NodePaddingVertical = sys::StyleVar_StyleVar_NodePaddingVertical,
}

pub struct StyleVarToken {
    ended: bool,
}
impl StyleVarToken {
    pub fn end(mut self) {
        self.ended = true;
        unsafe { sys::imnodes_PopStyleVar() };
    }
}

impl Drop for StyleVarToken {
    fn drop(&mut self) {
        if !self.ended {
            panic!("did not call end on a style var token");
        }
    }
}

#[must_use = "need to call end on StyleVarToken befor going out of scope"]
pub fn push_style_vare(style: StyleVar, value: f32, _: &Context) -> StyleVarToken {
    unsafe { sys::imnodes_PushStyleVar(style as i32, value) };
    StyleVarToken { ended: false }
}

#[derive(Clone)]
#[repr(i32)]
pub enum StyleFlag {
    None = sys::StyleFlags_StyleFlags_None,
    NodeOutline = sys::StyleFlags_StyleFlags_NodeOutline,
    GridLines = sys::StyleFlags_StyleFlags_GridLines,
}

#[derive(Clone)]
#[repr(i32)]
pub enum PinShape {
    Circle = sys::PinShape_PinShape_Circle,
    CircleFilled = sys::PinShape_PinShape_CircleFilled,
    Triangle = sys::PinShape_PinShape_Triangle,
    TriangleFilled = sys::PinShape_PinShape_TriangleFilled,
    Quad = sys::PinShape_PinShape_Quad,
    QuadFilled = sys::PinShape_PinShape_QuadFilled,
}

#[derive(Clone)]
#[repr(i32)]
pub enum AttributeFlag {
    None = sys::AttributeFlags_AttributeFlags_None,
    EnableLinkDetachWithDragClick =
        sys::AttributeFlags_AttributeFlags_EnableLinkDetachWithDragClick,
    EnableLinkCreationOnSnap = sys::AttributeFlags_AttributeFlags_EnableLinkCreationOnSnap,
}

pub struct AttributeFlagToken {
    ended: bool,
}
impl AttributeFlagToken {
    pub fn end(mut self) {
        self.ended = true;
        unsafe { sys::imnodes_PopAttributeFlag() };
    }
}

impl Drop for AttributeFlagToken {
    fn drop(&mut self) {
        if !self.ended {
            panic!("did not call end on a style var token");
        }
    }
}

#[must_use = "need to call end on AttributeFlagsToken befor going out of scope"]
pub fn push_attribute_flag(flag: AttributeFlag, _: &Context) -> AttributeFlagToken {
    unsafe { sys::imnodes_PushAttributeFlag(flag as i32) };
    AttributeFlagToken { ended: false }
}

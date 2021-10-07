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
    NodeBackground = sys::ImNodesCol__ImNodesCol_NodeBackground,
    NodeBackgroundHovered = sys::ImNodesCol__ImNodesCol_NodeBackgroundHovered,
    NodeBackgroundSelected = sys::ImNodesCol__ImNodesCol_NodeBackgroundSelected,
    NodeOutline = sys::ImNodesCol__ImNodesCol_NodeOutline,
    TitleBar = sys::ImNodesCol__ImNodesCol_TitleBar,
    TitleBarHovered = sys::ImNodesCol__ImNodesCol_TitleBarHovered,
    TitleBarSelected = sys::ImNodesCol__ImNodesCol_TitleBarSelected,
    Link = sys::ImNodesCol__ImNodesCol_Link,
    LinkHovered = sys::ImNodesCol__ImNodesCol_LinkHovered,
    LinkSelected = sys::ImNodesCol__ImNodesCol_LinkSelected,
    Pin = sys::ImNodesCol__ImNodesCol_Pin,
    PinHovered = sys::ImNodesCol__ImNodesCol_PinHovered,
    BoxSelector = sys::ImNodesCol__ImNodesCol_BoxSelector,
    BoxSelectorOutline = sys::ImNodesCol__ImNodesCol_BoxSelectorOutline,
    GridBackground = sys::ImNodesCol__ImNodesCol_GridBackground,
    GridLine = sys::ImNodesCol__ImNodesCol_GridLine,
    MiniMapBackground = sys::ImNodesCol__ImNodesCol_MiniMapBackground,
    MiniMapBackgroundHovered = sys::ImNodesCol__ImNodesCol_MiniMapBackgroundHovered,
    MiniMapOutline = sys::ImNodesCol__ImNodesCol_MiniMapOutline,
    MiniMapOutlineHovered = sys::ImNodesCol__ImNodesCol_MiniMapOutlineHovered,
    MiniMapNodeBackground = sys::ImNodesCol__ImNodesCol_MiniMapNodeBackground,
    MiniMapNodeBackgroundHovered = sys::ImNodesCol__ImNodesCol_MiniMapNodeBackgroundHovered,
    MiniMapNodeBackgroundSelected = sys::ImNodesCol__ImNodesCol_MiniMapNodeBackgroundSelected,
    MiniMapNodeOutline = sys::ImNodesCol__ImNodesCol_MiniMapNodeOutline,
    MiniMapLink = sys::ImNodesCol__ImNodesCol_MiniMapLink,
    MiniMapLinkSelected = sys::ImNodesCol__ImNodesCol_MiniMapLinkSelected,
    COUNT = sys::ImNodesCol__ImNodesCol_COUNT,
}

impl ColorStyle {
    pub const COUNT: u32 = sys::ImNodesCol__ImNodesCol_COUNT;

    #[must_use = "need to call pop on ColorToken befor going out of scope"]
    #[doc(alias = "PushColorStyle")]
    pub fn push_color<C: Into<ImColor32>>(self, color: C, _: &EditorContext) -> ColorToken {
        let color: ImColor32 = color.into();
        unsafe { sys::imnodes_PushColorStyle(self as i32, color.into()) };
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
        assert!(self.ended, "did not call pop on a color token");
    }
}

/// Location of the MiniMap
/// TODO add link to add_mini_map
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum MiniMapLocation {
    BottomLeft = sys::ImNodesMiniMapLocation__ImNodesMiniMapLocation_BottomLeft,
    BottomRight = sys::ImNodesMiniMapLocation__ImNodesMiniMapLocation_BottomRight,
    TopLeft = sys::ImNodesMiniMapLocation__ImNodesMiniMapLocation_TopLeft,
    TopRight = sys::ImNodesMiniMapLocation__ImNodesMiniMapLocation_TopRight,
}

/// The default size of each pin shape is balanced to occupy approximately the same surface area on the screen.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum StyleVar {
    GridSpacing = sys::ImNodesStyleVar__ImNodesStyleVar_GridSpacing,
    NodeCornerRounding = sys::ImNodesStyleVar__ImNodesStyleVar_NodeCornerRounding,
    NodePaddingHorizontal = sys::ImNodesStyleVar__ImNodesStyleVar_NodePaddingHorizontal,
    NodePaddingVertical = sys::ImNodesStyleVar__ImNodesStyleVar_NodePaddingVertical,
    NodeBorderThickness = sys::ImNodesStyleVar__ImNodesStyleVar_NodeBorderThickness,
    LinkThickness = sys::ImNodesStyleVar__ImNodesStyleVar_LinkThickness,
    LinkLineSegmentsPerLength = sys::ImNodesStyleVar__ImNodesStyleVar_LinkLineSegmentsPerLength,
    LinkHoverDistance = sys::ImNodesStyleVar__ImNodesStyleVar_LinkHoverDistance,
    /// The circle radius used when the pin shape is either [PinShape::Circle] or [PinShape::CircleFilled].
    PinCircleRadius = sys::ImNodesStyleVar__ImNodesStyleVar_PinCircleRadius,
    /// The quad side length used when the shape is either [PinShape::Quad] or [PinShape::QuadFilled].
    PinQuadSideLength = sys::ImNodesStyleVar__ImNodesStyleVar_PinQuadSideLength,
    /// The equilateral triangle side length used when the pin shape is either [PinShape::Triangle] or [PinShape::TriangleFilled].
    PinTriangleSideLength = sys::ImNodesStyleVar__ImNodesStyleVar_PinTriangleSideLength,
    /// The thickness of the line used when the pin shape is not filled.
    PinLineThickness = sys::ImNodesStyleVar__ImNodesStyleVar_PinLineThickness,
    /// The radius from the pin's center position inside of which it is detected as being hovered over.
    PinHoverRadius = sys::ImNodesStyleVar__ImNodesStyleVar_PinHoverRadius,
    /// Offsets the pins' positions from the edge of the node to the outside of the node.
    PinOffset = sys::ImNodesStyleVar__ImNodesStyleVar_PinOffset,
}

impl StyleVar {
    #[must_use = "need to call pop on StyleVarToken befor going out of scope"]
    #[doc(alias = "PushStyleVar")]
    pub fn push_val(self, value: f32, _: &EditorContext) -> StyleVarToken {
        unsafe { sys::imnodes_PushStyleVar(self as i32, value) };
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
        assert!(self.ended, "did not call pop on a style var token");
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum StyleFlag {
    // None = sys::StyleFlags_StyleFlags_None,
    NodeOutline = sys::ImNodesStyleFlags__ImNodesStyleFlags_NodeOutline,
    GridLines = sys::ImNodesStyleFlags__ImNodesStyleFlags_GridLines,
}

/// This enum controls the way attribute pins look.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum PinShape {
    Circle = sys::ImNodesPinShape__ImNodesPinShape_Circle,
    CircleFilled = sys::ImNodesPinShape__ImNodesPinShape_CircleFilled,
    Triangle = sys::ImNodesPinShape__ImNodesPinShape_Triangle,
    TriangleFilled = sys::ImNodesPinShape__ImNodesPinShape_TriangleFilled,
    Quad = sys::ImNodesPinShape__ImNodesPinShape_Quad,
    QuadFilled = sys::ImNodesPinShape__ImNodesPinShape_QuadFilled,
}

/// This enum controls the way the attribute pins behave.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum AttributeFlag {
    None = sys::ImNodesAttributeFlags__ImNodesAttributeFlags_None,
    /// Allow detaching a link by left-clicking and dragging the link at a pin it is connected to.
    /// NOTE: the user has to actually delete the link for this to work. A deleted link can be
    /// detected by calling [crate::LinkId::is_removed()] after [crate::scopes::editor()].
    EnableLinkDetachWithDragClick =
        sys::ImNodesAttributeFlags__ImNodesAttributeFlags_EnableLinkDetachWithDragClick,
    /// Visual snapping of an in progress link will trigger IsLink Created/Destroyed events. Allows
    /// for previewing the creation of a link while dragging it across attributes. See here for demo:
    /// <https://github.com/Nelarius/imnodes/issues/41#issuecomment-647132113> NOTE: the user has to
    /// actually delete the link for this to work. A deleted link can be detected by calling
    /// [crate::LinkId::is_removed()] after [crate::scopes::editor()].
    EnableLinkCreationOnSnap =
        sys::ImNodesAttributeFlags__ImNodesAttributeFlags_EnableLinkCreationOnSnap,
}

impl EditorContext {
    /// Push a single AttributeFlags value. By default, only AttributeFlags_None is set.
    #[must_use = "need to call pop on AttributeFlagsToken befor going out of scope"]
    #[doc(alias = "PushAttributeFlag")]
    pub fn push(&self, flag: AttributeFlag) -> AttributeFlagToken {
        unsafe { sys::imnodes_PushAttributeFlag(flag as i32) };
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
        assert!(self.ended, "did not call pop on a style var token");
    }
}

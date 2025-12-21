use crate::{EditorContext, sys};

/// Represents the visual style of the node editor.
///
/// This struct is a pure Rust representation of `imnodes_Style`.
/// You can modify the global style instance by passing this struct to [`EditorContext::set_style`].
///
/// Use `..Default::default()` to initialize with default values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// Spacing between grid lines.
    pub grid_spacing: f32,
    /// Corner rounding radius for nodes.
    pub node_corner_rounding: f32,
    /// Padding inside nodes.
    pub node_padding: [f32; 2],
    /// Thickness of node borders.
    pub node_border_thickness: f32,
    /// Thickness of links between pins.
    pub link_thickness: f32,
    /// Number of line segments per unit length for links.
    pub link_line_segments_per_length: f32,
    /// Distance threshold for link hover detection.
    pub link_hover_distance: f32,
    /// The circle radius used when the pin shape is [`PinShape::Circle`] or [`PinShape::CircleFilled`].
    pub pin_circle_radius: f32,
    /// The quad side length used when the shape is [`PinShape::Quad`] or [`PinShape::QuadFilled`].
    pub pin_quad_side_length: f32,
    /// The equilateral triangle side length used when the pin shape is [`PinShape::Triangle`] or [`PinShape::TriangleFilled`].
    pub pin_triangle_side_length: f32,
    /// The thickness of the line used when the pin shape is not filled (e.g., [`PinShape::Circle`]).
    pub pin_line_thickness: f32,
    /// The radius from the pin's center position within which it is detected as being hovered over.
    pub pin_hover_radius: f32,
    /// Offsets the pins' positions horizontally from the edge of the node. Positive values offset outwards.
    pub pin_offset: f32,
    /// Padding inside the minimap canvas.
    pub mini_map_padding: [f32; 2],
    /// Offset of the minimap from the specified corner.
    pub mini_map_offset: [f32; 2],
    /// Flags controlling boolean style options.
    pub flags: StyleFlags, // Use StyleFlags directly
    /// Color configuration for the style.
    pub colors: [u32; ColorStyle::COUNT as usize],
}

impl Default for Style {
    fn default() -> Self {
        let mut sys_style = sys::ImNodesStyle {
            GridSpacing: 24.0,
            NodeCornerRounding: 4.0,
            NodePadding: sys::ImVec2_c { x: 8.0, y: 8.0 },
            NodeBorderThickness: 1.0,
            LinkThickness: 3.0,
            LinkLineSegmentsPerLength: 0.1,
            LinkHoverDistance: 10.0,
            PinCircleRadius: 4.0,
            PinQuadSideLength: 7.0,
            PinTriangleSideLength: 9.5,
            PinLineThickness: 1.0,
            PinHoverRadius: 10.0,
            PinOffset: 0.0,
            MiniMapPadding: sys::ImVec2_c { x: 8.0, y: 8.0 },
            MiniMapOffset: sys::ImVec2_c { x: 4.0, y: 4.0 },
            Flags: (sys::ImNodesStyleFlags__ImNodesStyleFlags_GridLines
                | sys::ImNodesStyleFlags__ImNodesStyleFlags_NodeOutline) as i32,
            Colors: [0; sys::ImNodesCol__ImNodesCol_COUNT as usize],
        };
        unsafe {
            sys::imnodes_StyleColorsDark(&mut sys_style);
        }
        Self::from_sys(sys_style)
    }
}

impl Style {
    pub(crate) fn from_sys(sys: sys::ImNodesStyle) -> Self {
        Self {
            grid_spacing: sys.GridSpacing,
            node_corner_rounding: sys.NodeCornerRounding,
            node_padding: [sys.NodePadding.x, sys.NodePadding.y],
            node_border_thickness: sys.NodeBorderThickness,
            link_thickness: sys.LinkThickness,
            link_line_segments_per_length: sys.LinkLineSegmentsPerLength,
            link_hover_distance: sys.LinkHoverDistance,
            pin_circle_radius: sys.PinCircleRadius,
            pin_quad_side_length: sys.PinQuadSideLength,
            pin_triangle_side_length: sys.PinTriangleSideLength,
            pin_line_thickness: sys.PinLineThickness,
            pin_hover_radius: sys.PinHoverRadius,
            pin_offset: sys.PinOffset,
            mini_map_padding: [sys.MiniMapPadding.x, sys.MiniMapPadding.y],
            mini_map_offset: [sys.MiniMapOffset.x, sys.MiniMapOffset.y],
            flags: StyleFlags::from_bits_truncate(sys.Flags),
            colors: sys.Colors,
        }
    }

    pub(crate) fn to_sys(self) -> sys::ImNodesStyle {
        sys::ImNodesStyle {
            GridSpacing: self.grid_spacing,
            NodeCornerRounding: self.node_corner_rounding,
            NodePadding: sys::ImVec2_c {
                x: self.node_padding[0],
                y: self.node_padding[1],
            },
            NodeBorderThickness: self.node_border_thickness,
            LinkThickness: self.link_thickness,
            LinkLineSegmentsPerLength: self.link_line_segments_per_length,
            LinkHoverDistance: self.link_hover_distance,
            PinCircleRadius: self.pin_circle_radius,
            PinQuadSideLength: self.pin_quad_side_length,
            PinTriangleSideLength: self.pin_triangle_side_length,
            PinLineThickness: self.pin_line_thickness,
            PinHoverRadius: self.pin_hover_radius,
            PinOffset: self.pin_offset,
            MiniMapPadding: sys::ImVec2_c {
                x: self.mini_map_padding[0],
                y: self.mini_map_padding[1],
            },
            MiniMapOffset: sys::ImVec2_c {
                x: self.mini_map_offset[0],
                y: self.mini_map_offset[1],
            },
            Flags: self.flags.bits(),
            Colors: self.colors,
        }
    }
}

/// Provides methods for manipulating the editor's style.
impl<'a> EditorContext<'a> {
    /// Sets the style of the current editor context.
    /// This copies the values from the provided Rust struct to the internal C struct.
    pub fn set_style(&mut self, style: &Style) {
        let _ = self.set_as_current_editor();
        let sys_style = unsafe { &mut *(sys::imnodes_GetStyle()) };
        *sys_style = style.to_sys();
    }

    /// Gets a copy of the current style of the editor context.
    pub fn get_style(&mut self) -> Style {
        let _ = self.set_as_current_editor();
        let sys_style = unsafe { &*(sys::imnodes_GetStyle()) };
        Style::from_sys(*sys_style)
    }
}

/// Identifies a specific color setting within the node editor's style.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum ColorStyle {
    /// Node background color.
    NodeBackground = sys::ImNodesCol__ImNodesCol_NodeBackground,
    /// Node background color when hovered.
    NodeBackgroundHovered = sys::ImNodesCol__ImNodesCol_NodeBackgroundHovered,
    /// Node background color when selected.
    NodeBackgroundSelected = sys::ImNodesCol__ImNodesCol_NodeBackgroundSelected,
    /// Node outline color (if `StyleFlags::NODE_OUTLINE` is enabled).
    NodeOutline = sys::ImNodesCol__ImNodesCol_NodeOutline,
    /// Node title bar color.
    TitleBar = sys::ImNodesCol__ImNodesCol_TitleBar,
    /// Node title bar color when hovered.
    TitleBarHovered = sys::ImNodesCol__ImNodesCol_TitleBarHovered,
    /// Node title bar color when selected.
    TitleBarSelected = sys::ImNodesCol__ImNodesCol_TitleBarSelected,
    /// Link color.
    Link = sys::ImNodesCol__ImNodesCol_Link,
    /// Link color when hovered.
    LinkHovered = sys::ImNodesCol__ImNodesCol_LinkHovered,
    /// Link color when selected.
    LinkSelected = sys::ImNodesCol__ImNodesCol_LinkSelected,
    /// Pin color.
    Pin = sys::ImNodesCol__ImNodesCol_Pin,
    /// Pin color when hovered.
    PinHovered = sys::ImNodesCol__ImNodesCol_PinHovered,
    /// Box selector background color.
    BoxSelector = sys::ImNodesCol__ImNodesCol_BoxSelector,
    /// Box selector outline color.
    BoxSelectorOutline = sys::ImNodesCol__ImNodesCol_BoxSelectorOutline,
    /// Editor background grid color.
    GridBackground = sys::ImNodesCol__ImNodesCol_GridBackground,
    /// Editor grid line color.
    GridLine = sys::ImNodesCol__ImNodesCol_GridLine,
    /// Editor primary grid line color (thicker lines).
    GridLinePrimary = sys::ImNodesCol__ImNodesCol_GridLinePrimary,
    /// Minimap background color.
    MiniMapBackground = sys::ImNodesCol__ImNodesCol_MiniMapBackground,
    /// Minimap background color when hovered.
    MiniMapBackgroundHovered = sys::ImNodesCol__ImNodesCol_MiniMapBackgroundHovered,
    /// Minimap outline color.
    MiniMapOutline = sys::ImNodesCol__ImNodesCol_MiniMapOutline,
    /// Minimap outline color when hovered.
    MiniMapOutlineHovered = sys::ImNodesCol__ImNodesCol_MiniMapOutlineHovered,
    /// Minimap node background color.
    MiniMapNodeBackground = sys::ImNodesCol__ImNodesCol_MiniMapNodeBackground,
    /// Minimap node background color when hovered.
    MiniMapNodeBackgroundHovered = sys::ImNodesCol__ImNodesCol_MiniMapNodeBackgroundHovered,
    /// Minimap node background color when selected.
    MiniMapNodeBackgroundSelected = sys::ImNodesCol__ImNodesCol_MiniMapNodeBackgroundSelected,
    /// Minimap node outline color.
    MiniMapNodeOutline = sys::ImNodesCol__ImNodesCol_MiniMapNodeOutline,
    /// Minimap link color.
    MiniMapLink = sys::ImNodesCol__ImNodesCol_MiniMapLink,
    /// Minimap link color when selected.
    MiniMapLinkSelected = sys::ImNodesCol__ImNodesCol_MiniMapLinkSelected,
    /// Minimap canvas background color.
    MiniMapCanvas = sys::ImNodesCol__ImNodesCol_MiniMapCanvas,
    /// Minimap canvas outline color.
    MiniMapCanvasOutline = sys::ImNodesCol__ImNodesCol_MiniMapCanvasOutline,
}

impl ColorStyle {
    /// The total number of distinct color style settings.
    pub const COUNT: u32 = sys::ImNodesCol__ImNodesCol_COUNT;
}

/// Identifies a style variable and its value to be pushed onto the stack.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StyleVar {
    /// Spacing between grid lines.
    GridSpacing(f32),
    /// Corner rounding radius for nodes.
    NodeCornerRounding(f32),
    /// Padding inside nodes.
    NodePadding([f32; 2]),
    /// Thickness of node borders.
    NodeBorderThickness(f32),
    /// Thickness of links between pins.
    LinkThickness(f32),
    /// Number of line segments used to render links per unit of length.
    LinkLineSegmentsPerLength(f32),
    /// Distance threshold for detecting link hovering.
    LinkHoverDistance(f32),
    /// The circle radius used when the pin shape is [`PinShape::Circle`] or [`PinShape::CircleFilled`].
    PinCircleRadius(f32),
    /// The quad side length used when the shape is [`PinShape::Quad`] or [`PinShape::QuadFilled`].
    PinQuadSideLength(f32),
    /// The equilateral triangle side length used when the pin shape is [`PinShape::Triangle`] or [`PinShape::TriangleFilled`].
    PinTriangleSideLength(f32),
    /// The thickness of the line used when the pin shape is not filled (e.g., [`PinShape::Circle`]).
    PinLineThickness(f32),
    /// The radius from the pin's center position within which it is detected as being hovered over.
    PinHoverRadius(f32),
    /// Offsets the pins' positions horizontally from the edge of the node.
    PinOffset(f32),
    /// Padding inside the minimap canvas.
    MiniMapPadding([f32; 2]),
    /// Offset of the minimap from the specified corner.
    MiniMapOffset([f32; 2]),
}

/// Flags controlling boolean style options for the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StyleFlags(pub i32);

impl StyleFlags {
    /// No flags set.
    pub const NONE: Self = Self(sys::ImNodesStyleFlags__ImNodesStyleFlags_None as i32);
    /// Draw outlines around nodes.
    pub const NODE_OUTLINE: Self =
        Self(sys::ImNodesStyleFlags__ImNodesStyleFlags_NodeOutline as i32);
    /// Draw grid lines in the background.
    pub const GRID_LINES: Self = Self(sys::ImNodesStyleFlags__ImNodesStyleFlags_GridLines as i32);
    /// Draw primary grid lines (at multiples of `GridSpacing * 10.0`) thicker.
    pub const GRID_LINES_PRIMARY: Self =
        Self(sys::ImNodesStyleFlags__ImNodesStyleFlags_GridLinesPrimary as i32);
    /// Enable snapping nodes to the grid when dragging. Requires [`StyleFlags::GRID_LINES`].
    pub const GRID_SNAPPING: Self =
        Self(sys::ImNodesStyleFlags__ImNodesStyleFlags_GridSnapping as i32);

    /// Returns the underlying integer value.
    pub fn bits(&self) -> i32 {
        self.0
    }

    /// Creates a `StyleFlags` from an integer, truncating any unknown bits.
    pub fn from_bits_truncate(bits: i32) -> Self {
        Self(bits)
    }

    /// Returns `true` if the other flags are contained within `self`.
    pub fn contains(&self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    /// Inserts the specified flags in-place.
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    /// Removes the specified flags in-place.
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

impl std::ops::BitOr for StyleFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for StyleFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for StyleFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitAndAssign for StyleFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl std::ops::Not for StyleFlags {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/// Controls the visual shape of attribute pins.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum PinShape {
    /// A simple circle outline.
    Circle = sys::ImNodesPinShape__ImNodesPinShape_Circle,
    /// A filled circle.
    CircleFilled = sys::ImNodesPinShape__ImNodesPinShape_CircleFilled,
    /// A simple triangle outline.
    Triangle = sys::ImNodesPinShape__ImNodesPinShape_Triangle,
    /// A filled triangle.
    TriangleFilled = sys::ImNodesPinShape__ImNodesPinShape_TriangleFilled,
    /// A simple square outline.
    Quad = sys::ImNodesPinShape__ImNodesPinShape_Quad,
    /// A filled square.
    QuadFilled = sys::ImNodesPinShape__ImNodesPinShape_QuadFilled,
}

/// Flags controlling the behavior of individual attributes (pins).
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)]
pub enum AttributeFlags {
    /// Default behavior.
    None = sys::ImNodesAttributeFlags__ImNodesAttributeFlags_None as i32,
    /// Allows detaching existing links by clicking and dragging the link at the pin it is connected to.
    EnableLinkDetachWithDragClick =
        sys::ImNodesAttributeFlags__ImNodesAttributeFlags_EnableLinkDetachWithDragClick as i32,
    /// Allows creation of links by simply snapping the dragged link end onto a compatible pin.
    EnableLinkCreationOnSnap =
        sys::ImNodesAttributeFlags__ImNodesAttributeFlags_EnableLinkCreationOnSnap as i32,
}

/// Specifies the corner location of the minimap within the editor canvas.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum MiniMapLocation {
    /// Bottom-left corner.
    BottomLeft = sys::ImNodesMiniMapLocation__ImNodesMiniMapLocation_BottomLeft,
    /// Bottom-right corner.
    BottomRight = sys::ImNodesMiniMapLocation__ImNodesMiniMapLocation_BottomRight,
    /// Top-left corner.
    TopLeft = sys::ImNodesMiniMapLocation__ImNodesMiniMapLocation_TopLeft,
    /// Top-right corner.
    TopRight = sys::ImNodesMiniMapLocation__ImNodesMiniMapLocation_TopRight,
}

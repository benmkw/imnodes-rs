use crate::{EditorContext, sys};
use imgui::ImColor32;

// Re-export the underlying sys type for IO

/// Wrapper struct for `imnodes_sys::ImNodesStyle` to implement `Default` locally.
/// This works around Rust's orphan rule (E0117).
#[derive(Debug, Clone)] // Removed PartialEq due to C array inside sys::ImNodesStyle
#[repr(transparent)]
pub struct Style(sys::ImNodesStyle);

/// Represents the visual style of the node editor.
///
/// You can modify the global style instance by calling [`EditorContext::get_style`].
/// To get a default style configuration (e.g., for copying or comparison),
/// use [`Style::default()`].
impl Default for Style {
    /// Creates a default style configuration based on imnodes' internal defaults.
    ///
    /// This initializes style variables and applies the dark color theme
    /// (`imnodes_StyleColorsDark`).
    fn default() -> Self {
        let mut style = sys::ImNodesStyle {
            GridSpacing: 24.0,
            NodeCornerRounding: 4.0,
            NodePadding: sys::ImVec2 { x: 8.0, y: 8.0 },
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
            MiniMapPadding: sys::ImVec2 { x: 8.0, y: 8.0 },
            MiniMapOffset: sys::ImVec2 { x: 4.0, y: 4.0 },
            Flags: (StyleFlags::GridLines as i32) | (StyleFlags::NodeOutline as i32),
            // Initialize colors array temporarily
            Colors: [0; sys::ImNodesCol__ImNodesCol_COUNT as usize],
        };
        // Safety: Call the C function to populate the Colors array with the dark theme.
        unsafe {
            sys::imnodes_StyleColorsDark(&mut style);
        }
        Style(style)
    }
}

#[deprecated = "Use `imnodes::Style::default()` instead."]
#[must_use]
/// Creates an `ImNodesStyle` struct initialized with default values and the dark color theme.
/// Deprecated: Use [`imnodes::Style::default()`] instead.
///
/// # Example
/// ```no_run
/// # use imnodes::Style;
/// // Old way (deprecated):
/// // let style = imnodes::create_imnodes_style();
/// // New way:
/// let style = Style::default();
/// ```
pub fn create_imnodes_style() -> sys::ImNodesStyle {
    Style::default().0 // Return the inner sys::ImNodesStyle
}

/// Provides methods for manipulating the editor's style.
impl EditorContext {
    /// Applies the dark color theme to the provided style struct.
    #[doc(alias = "StyleColorsDark")]
    pub fn set_style_colors_dark(&self, style: &mut Style) -> &Self {
        // Safety: C API call. Modifies the passed style struct.
        unsafe { sys::imnodes_StyleColorsDark(&mut style.0) };
        self
    }

    /// Applies the classic color theme to the provided style struct.
    #[doc(alias = "StyleColorsClassic")]
    pub fn set_style_colors_classic(&self, style: &mut Style) -> &Self {
        // Safety: C API call. Modifies the passed style struct.
        unsafe { sys::imnodes_StyleColorsClassic(&mut style.0) };
        self
    }

    /// Applies the light color theme to the provided style struct.
    #[doc(alias = "StyleColorsLight")]
    pub fn set_style_colors_light(&self, style: &mut Style) -> &Self {
        // Safety: C API call. Modifies the passed style struct.
        unsafe { sys::imnodes_StyleColorsLight(&mut style.0) };
        self
    }
}

/// Identifies a specific color setting within the node editor's style.
///
/// Used with [`ColorStyle::push_color`] and [`ColorToken::pop`].
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum ColorStyle {
    /// Node background color.
    NodeBackground = sys::ImNodesCol__ImNodesCol_NodeBackground,
    /// Node background color when hovered.
    NodeBackgroundHovered = sys::ImNodesCol__ImNodesCol_NodeBackgroundHovered,
    /// Node background color when selected.
    NodeBackgroundSelected = sys::ImNodesCol__ImNodesCol_NodeBackgroundSelected,
    /// Node outline color (if `StyleFlags::NodeOutline` is enabled).
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
    /// Total number of color styles.
    COUNT = sys::ImNodesCol__ImNodesCol_COUNT,
}

impl ColorStyle {
    /// The total number of distinct color style settings.
    pub const COUNT: u32 = sys::ImNodesCol__ImNodesCol_COUNT;

    /// Pushes a color onto the style stack for this specific `ColorStyle` item.
    ///
    /// The change applies until the returned [`ColorToken`] is popped.
    /// Remember to call `.pop()` on the token before it goes out of scope.
    #[doc(alias = "PushColorStyle")]
    #[must_use = "The returned ColorToken must be popped to restore the previous color"]
    pub fn push_color<C: Into<ImColor32>>(self, color: C, _context: &EditorContext) -> ColorToken {
        let color: ImColor32 = color.into();
        // Safety: C API call. Pushes a color onto the internal stack.
        unsafe { sys::imnodes_PushColorStyle(self as i32, color.into()) };
        ColorToken { ended: false }
    }
}

/// A token representing a pushed color style change.
///
/// Must be popped using [`ColorToken::pop`] before it goes out of scope to maintain
/// the integrity of the style stack. Dropping without popping will cause a panic.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ColorToken {
    ended: bool,
}

impl ColorToken {
    /// Pops the color style change associated with this token from the stack, restoring the previous color.
    #[doc(alias = "PopColorStyle")]
    pub fn pop(mut self) {
        // Prevent Drop::drop from panicking
        self.ended = true;
        // Safety: C API call. Pops one item from the color style stack.
        unsafe { sys::imnodes_PopColorStyle() };
    }
}

impl Drop for ColorToken {
    /// Panics if the token is dropped without being popped.
    fn drop(&mut self) {
        assert!(
            self.ended,
            "`ColorToken` was dropped without calling `pop()`. This likely means a color style was pushed but not popped, leading to an incorrect style stack."
        );
    }
}

/// Specifies the corner location of the minimap within the editor canvas.
/// Used with [`crate::EditorScope::add_mini_map`].
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

/// Identifies a specific style variable setting within the node editor's style.
///
/// Use [`StyleVar::push_f32`] or [`StyleVar::push_vec2`] to modify these temporarily.
/// The default size of each pin shape is balanced to occupy approximately the same surface area on the screen.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum StyleVar {
    /// float: Spacing between grid lines.
    GridSpacing = sys::ImNodesStyleVar__ImNodesStyleVar_GridSpacing,
    /// float: Corner rounding radius for nodes.
    NodeCornerRounding = sys::ImNodesStyleVar__ImNodesStyleVar_NodeCornerRounding,
    /// ImVec2: Padding inside nodes. Use [`StyleVar::push_vec2`].
    NodePadding = sys::ImNodesStyleVar__ImNodesStyleVar_NodePadding,
    /// float: Thickness of node borders.
    NodeBorderThickness = sys::ImNodesStyleVar__ImNodesStyleVar_NodeBorderThickness,
    /// float: Thickness of links between pins.
    LinkThickness = sys::ImNodesStyleVar__ImNodesStyleVar_LinkThickness,
    /// float: Number of line segments used to render links per unit of length. Higher values result in smoother curves.
    LinkLineSegmentsPerLength = sys::ImNodesStyleVar__ImNodesStyleVar_LinkLineSegmentsPerLength,
    /// float: Distance threshold for detecting link hovering.
    LinkHoverDistance = sys::ImNodesStyleVar__ImNodesStyleVar_LinkHoverDistance,
    /// float: The circle radius used when the pin shape is [`PinShape::Circle`] or [`PinShape::CircleFilled`].
    PinCircleRadius = sys::ImNodesStyleVar__ImNodesStyleVar_PinCircleRadius,
    /// float: The quad side length used when the shape is [`PinShape::Quad`] or [`PinShape::QuadFilled`].
    PinQuadSideLength = sys::ImNodesStyleVar__ImNodesStyleVar_PinQuadSideLength,
    /// float: The equilateral triangle side length used when the pin shape is [`PinShape::Triangle`] or [`PinShape::TriangleFilled`].
    PinTriangleSideLength = sys::ImNodesStyleVar__ImNodesStyleVar_PinTriangleSideLength,
    /// float: The thickness of the line used when the pin shape is not filled (e.g., [`PinShape::Circle`]).
    PinLineThickness = sys::ImNodesStyleVar__ImNodesStyleVar_PinLineThickness,
    /// float: The radius from the pin's center position within which it is detected as being hovered over.
    PinHoverRadius = sys::ImNodesStyleVar__ImNodesStyleVar_PinHoverRadius,
    /// float: Offsets the pins' positions horizontally from the edge of the node. Positive values offset outwards.
    PinOffset = sys::ImNodesStyleVar__ImNodesStyleVar_PinOffset,
    /// ImVec2: Padding inside the minimap canvas. Use [`StyleVar::push_vec2`].
    MiniMapPadding = sys::ImNodesStyleVar__ImNodesStyleVar_MiniMapPadding,
    /// ImVec2: Offset of the minimap from the specified corner. Use [`StyleVar::push_vec2`].
    MiniMapOffset = sys::ImNodesStyleVar__ImNodesStyleVar_MiniMapOffset,
    /// Number of style variables.
    COUNT = sys::ImNodesStyleVar__ImNodesStyleVar_COUNT,
}

impl StyleVar {
    /// Pushes a float value onto the style stack for this specific `StyleVar` item.
    ///
    /// The change applies until the returned [`StyleVarToken`] is popped.
    /// Remember to call `.pop(1)` on the token before it goes out of scope.
    /// Panics if this `StyleVar` expects an `ImVec2`.
    #[doc(alias = "PushStyleVar_Float")]
    #[must_use = "The returned StyleVarToken must be popped to restore the previous value"]
    pub fn push_f32(self, value: f32, _context: &EditorContext) -> StyleVarToken {
        match self {
            StyleVar::NodePadding | StyleVar::MiniMapPadding | StyleVar::MiniMapOffset => {
                panic!("StyleVar {self:?} expects an ImVec2, use push_vec2");
            }
            _ => {}
        }
        // Safety: C API call. Pushes a float onto the internal stack.
        unsafe { sys::imnodes_PushStyleVar_Float(self as i32, value) };
        StyleVarToken { ended: false }
    }

    /// Pushes an `ImVec2` value onto the style stack for this specific `StyleVar` item.
    ///
    /// The change applies until the returned [`StyleVarToken`] is popped.
    /// Remember to call `.pop(1)` on the token before it goes out of scope.
    /// Panics if this `StyleVar` expects a float.
    #[doc(alias = "PushStyleVar_Vec2")]
    #[must_use = "The returned StyleVarToken must be popped to restore the previous value"]
    pub fn push_vec2(self, value: sys::ImVec2, _context: &EditorContext) -> StyleVarToken {
        match self {
            StyleVar::NodePadding | StyleVar::MiniMapPadding | StyleVar::MiniMapOffset => {}
            _ => {
                panic!("StyleVar {self:?} expects a float, use push_f32");
            }
        }
        // Safety: C API call. Pushes an ImVec2 onto the internal stack.
        unsafe { sys::imnodes_PushStyleVar_Vec2(self as i32, value) };
        StyleVarToken { ended: false }
    }
}

/// A token representing a pushed style variable change (float or ImVec2).
///
/// Must be popped using [`StyleVarToken::pop`] before it goes out of scope to maintain
/// the integrity of the style stack. Dropping without popping will cause a panic.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StyleVarToken {
    ended: bool,
}
impl StyleVarToken {
    /// Pops the style variable change(s) associated with this token from the stack.
    ///
    /// # Arguments
    /// * `count`: The number of style variables to pop (usually 1).
    #[doc(alias = "PopStyleVar")]
    pub fn pop(mut self, count: i32) {
        assert!(count > 0, "Pop count must be positive");
        // Prevent Drop::drop from panicking
        self.ended = true;
        // Safety: C API call. Pops `count` items from the style var stack.
        unsafe { sys::imnodes_PopStyleVar(count) };
    }
}

impl Drop for StyleVarToken {
    /// Panics if the token is dropped without being popped.
    fn drop(&mut self) {
        assert!(
            self.ended,
            "`StyleVarToken` was dropped without calling `pop()`. This likely means a style variable was pushed but not popped, leading to an incorrect style stack."
        );
    }
}

/// Flags controlling boolean style options for the editor.
///
/// These flags are set in [`Style.0.Flags`]. Multiple flags can be combined using bitwise OR.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)] // Match the underlying C type ImNodesStyleFlags
pub enum StyleFlags {
    /// No flags set.
    None = sys::ImNodesStyleFlags__ImNodesStyleFlags_None as i32,
    /// Draw outlines around nodes.
    NodeOutline = sys::ImNodesStyleFlags__ImNodesStyleFlags_NodeOutline as i32,
    /// Draw grid lines in the background.
    GridLines = sys::ImNodesStyleFlags__ImNodesStyleFlags_GridLines as i32,
    /// Draw primary grid lines (at multiples of `GridSpacing * 10.0`) thicker.
    GridLinesPrimary = sys::ImNodesStyleFlags__ImNodesStyleFlags_GridLinesPrimary as i32,
    /// Enable snapping nodes to the grid when dragging. Requires [`StyleFlags::GridLines`].
    GridSnapping = sys::ImNodesStyleFlags__ImNodesStyleFlags_GridSnapping as i32,
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
///
/// These are pushed onto a stack using [`EditorContext::push_attribute_flag`]
/// and popped using [`AttributeFlagToken::pop`].
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(i32)] // Match the underlying C type ImNodesAttributeFlags
pub enum AttributeFlags {
    /// Default behavior.
    None = sys::ImNodesAttributeFlags__ImNodesAttributeFlags_None as i32,
    /// Allows detaching existing links by clicking and dragging the link at the pin it is connected to.
    ///
    /// Requires the user to handle the link destruction event detected by [`crate::LinkId::is_destroyed`].
    EnableLinkDetachWithDragClick =
        sys::ImNodesAttributeFlags__ImNodesAttributeFlags_EnableLinkDetachWithDragClick as i32,
    /// Allows creation of links by simply snapping the dragged link end onto a compatible pin.
    /// Triggers link creation/destruction events during the drag for preview.
    ///
    /// Requires the user to handle the link creation/destruction events.
    /// See: <https://github.com/Nelarius/imnodes/issues/41#issuecomment-647132113>
    EnableLinkCreationOnSnap =
        sys::ImNodesAttributeFlags__ImNodesAttributeFlags_EnableLinkCreationOnSnap as i32,
}

impl EditorContext {
    /// Pushes an [`AttributeFlags`] setting onto the internal stack.
    ///
    /// The flag applies to attributes created after this call, until the
    /// returned [`AttributeFlagToken`] is popped.
    /// By default, only `AttributeFlags::None` is active.
    ///
    /// Remember to call `.pop()` on the returned token.
    #[doc(alias = "PushAttributeFlag")]
    #[must_use = "The returned AttributeFlagToken must be popped to restore the previous flag state"]
    pub fn push_attribute_flag(&self, flag: AttributeFlags) -> AttributeFlagToken {
        // Safety: C API call. Pushes a flag onto the internal stack.
        unsafe { sys::imnodes_PushAttributeFlag(flag as i32) };
        AttributeFlagToken { ended: false }
    }
}

/// A token representing a pushed attribute flag change.
///
/// Must be popped using [`AttributeFlagToken::pop`] before it goes out of scope to maintain
/// the integrity of the attribute flag stack. Dropping without popping will cause a panic.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AttributeFlagToken {
    ended: bool,
}
impl AttributeFlagToken {
    /// Pops the attribute flag change associated with this token from the stack.
    #[doc(alias = "PopAttributeFlag")]
    pub fn pop(mut self) {
        // Prevent Drop::drop from panicking
        self.ended = true;
        // Safety: C API call. Pops one item from the attribute flag stack.
        unsafe { sys::imnodes_PopAttributeFlag() };
    }
}

impl Drop for AttributeFlagToken {
    /// Panics if the token is dropped without being popped.
    fn drop(&mut self) {
        assert!(
            self.ended,
            "`AttributeFlagToken` was dropped without calling `pop()`. This likely means an attribute flag was pushed but not popped, leading to incorrect behavior."
        );
    }
}

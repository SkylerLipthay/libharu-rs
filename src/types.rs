use haru;

/// Describes how a page should be displayed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PageLayout {
    /// The viewer application determines the layout.
    Default,
    /// Only one page is displayed.
    Single,
    /// Display the pages in one column.
    OneColumn,
    /// Display the pages in two column. The page of the odd number is displayed left.
    TwoColumnLeft,
    /// Display the pages in two column. The page of the odd number is displayed right.
    TwoColumnRight,
}

/// Converts a `PageLayout` to its corresponding internal layout code.
pub fn page_layout_as_int(layout: PageLayout) -> haru::HPDF_PageLayout {
    use haru::Enum__HPDF_PageLayout::*;

    match layout {
        PageLayout::Default => HPDF_PAGE_LAYOUT_EOF,
        PageLayout::Single => HPDF_PAGE_LAYOUT_SINGLE,
        PageLayout::OneColumn => HPDF_PAGE_LAYOUT_ONE_COLUMN,
        PageLayout::TwoColumnLeft => HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT,
        PageLayout::TwoColumnRight => HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT,
    }
}

/// Returns a `PageLayout` for the internal layout code.
///
/// # Panics
///
/// Panics if the layout mode is unrecognized or unsupported.
pub fn page_layout_from_int(layout: haru::HPDF_PageLayout) -> PageLayout {
    use haru::Enum__HPDF_PageLayout::*;

    match layout {
        HPDF_PAGE_LAYOUT_EOF => PageLayout::Default,
        HPDF_PAGE_LAYOUT_SINGLE => PageLayout::Single,
        HPDF_PAGE_LAYOUT_ONE_COLUMN => PageLayout::OneColumn,
        HPDF_PAGE_LAYOUT_TWO_COLUMN_LEFT => PageLayout::TwoColumnLeft,
        HPDF_PAGE_LAYOUT_TWO_COLUMN_RIGHT => PageLayout::TwoColumnRight,
        _ => panic!("Unrecognized or unsupported page layout setting"),
    }
}

/// A list of all types of stroke line caps.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineCap {
    /// Line is squared off at path endpoint.
    Butt,
    /// End of line becomes a semicircle whose center is at path endpoint.
    Round,
    /// Line continues beyond endpoint, goes on half the endpoint stroke width.
    ProjectingSquare,
}

/// Converts a `LineCap` to its corresponding internal layout code.
pub fn line_cap_as_int(line_cap: LineCap) -> haru::HPDF_LineCap {
    use haru::Enum__HPDF_LineCap::*;

    match line_cap {
        LineCap::Butt => HPDF_BUTT_END,
        LineCap::Round => HPDF_ROUND_END,
        // Boo! Sorry, did I `SCUARE` you?
        LineCap::ProjectingSquare => HPDF_PROJECTING_SCUARE_END,
    }
}

/// Returns a `LineCap` for the internal line cap code.
///
/// # Panics
///
/// Panics if the line cap mode is unrecognized or unsupported.
pub fn line_cap_from_int(line_cap: haru::HPDF_LineCap) -> LineCap {
    use haru::Enum__HPDF_LineCap::*;

    match line_cap {
        HPDF_BUTT_END => LineCap::Butt,
        HPDF_ROUND_END => LineCap::Round,
        HPDF_PROJECTING_SCUARE_END => LineCap::ProjectingSquare,
        _ => panic!("Unrecognized or unsupported line cap setting"),
    }
}

/// A list of all types of stroke line joins.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineJoin {
    /// A perfect, sharp joint. Produces a corner with one angle.
    Miter,
    /// A circular joint. Produces a corner with a theoretically infinite number of angles.
    Round,
    /// A bevelled joint. Produces a corner with two angles.
    Bevel,
}

/// Converts a `LineJoin` to its corresponding internal layout code.
pub fn line_join_as_int(line_join: LineJoin) -> haru::HPDF_LineJoin {
    use haru::Enum__HPDF_LineJoin::*;

    match line_join {
        LineJoin::Miter => HPDF_MITER_JOIN,
        LineJoin::Round => HPDF_ROUND_JOIN,
        LineJoin::Bevel => HPDF_BEVEL_JOIN,
    }
}

/// Returns a `LineJoin` for the internal line join code.
///
/// # Panics
///
/// Panics if the line join mode is unrecognized or unsupported.
pub fn line_join_from_int(line_join: haru::HPDF_LineJoin) -> LineJoin {
    use haru::Enum__HPDF_LineJoin::*;

    match line_join {
        HPDF_MITER_JOIN => LineJoin::Miter,
        HPDF_ROUND_JOIN => LineJoin::Round,
        HPDF_BEVEL_JOIN => LineJoin::Bevel,
        _ => panic!("Unrecognized or unsupported line join setting"),
    }
}

/// A point on a 2-dimensional plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// The x-axis magnitude.
    pub x: f32,
    /// The y-axis magnitude.
    pub y: f32,
}

impl Point {
    /// Creates a new point from a given coordinate pair.
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}

/// A size on a 2-dimensional plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    /// The width.
    pub width: f32,
    /// The height.
    pub height: f32,
}

impl Size {
    /// Creates a new size from a given dimensions.
    pub fn new(width: f32, height: f32) -> Size {
        Size { width: width, height: height }
    }
}

/// A list of all types of color spaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorSpace {
    DeviceGray,
    DeviceRgb,
    DeviceCmyk,
    CalGray,
    CalRgb,
    Lab,
    IccBased,
    Separation,
    DeviceN,
    Indexed,
    Pattern,
}

/// Returns a `ColorSpace` for the internal color space code.
///
/// # Panics
///
/// Panics if the color space code is unrecognized or unsupported.
pub fn color_space_from_int(color_space: haru::HPDF_ColorSpace) -> ColorSpace {
    use haru::Enum__HPDF_ColorSpace::*;

    match color_space {
        HPDF_CS_DEVICE_GRAY => ColorSpace::DeviceGray,
        HPDF_CS_DEVICE_RGB => ColorSpace::DeviceRgb,
        HPDF_CS_DEVICE_CMYK => ColorSpace::DeviceCmyk,
        HPDF_CS_CAL_GRAY => ColorSpace::CalGray,
        HPDF_CS_CAL_RGB => ColorSpace::CalRgb,
        HPDF_CS_LAB => ColorSpace::Lab,
        HPDF_CS_ICC_BASED => ColorSpace::IccBased,
        HPDF_CS_SEPARATION => ColorSpace::Separation,
        HPDF_CS_DEVICE_N => ColorSpace::DeviceN,
        HPDF_CS_INDEXED => ColorSpace::Indexed,
        HPDF_CS_PATTERN => ColorSpace::Pattern,
        _ => panic!("Unrecognized or unsupported color space setting"),
    }
}

/// Describes how text should be aligned when displayed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextAlignment {
    Left,
    Right,
    Center,
    Justify,
}

/// Converts a `TextAlignment` to its corresponding internal code.
pub fn text_alignment_as_int(layout: TextAlignment) -> haru::HPDF_TextAlignment {
    use haru::Enum__HPDF_TextAlignment::*;

    match layout {
        TextAlignment::Left => HPDF_TALIGN_LEFT,
        TextAlignment::Right => HPDF_TALIGN_RIGHT,
        TextAlignment::Center => HPDF_TALIGN_CENTER,
        TextAlignment::Justify => HPDF_TALIGN_JUSTIFY,
    }
}

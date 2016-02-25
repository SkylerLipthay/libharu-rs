use document::DocumentHandle;
use error::Error;
use font::{self, Font};
use haru;
use std::ffi::CString;
use std::ptr;
use std::rc::Rc;
use types::{self, ColorSpace, LineCap, LineJoin, Point, Size, TextAlignment};

/// A single page of a PDF document.
pub struct Page {
    handle: haru::HPDF_Page,
    // Keep a handle to the parent document to keep it from dropping while this `Page` is in scope.
    doc: Rc<DocumentHandle>,
}

impl Page {
    /// Returns the page width.
    ///
    /// The default value is `595.0`.
    pub fn width(&self) -> f32 {
        unsafe { haru::HPDF_Page_GetWidth(self.handle) }
    }

    /// Sets the page width.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_width(&mut self, width: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_SetWidth(self.handle, width) }));
        Ok(self)
    }

    /// Returns the page height.
    ///
    /// The default value is `841.0`.
    pub fn height(&self) -> f32 {
        unsafe { haru::HPDF_Page_GetHeight(self.handle) }
    }

    /// Sets the page height.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_height(&mut self, height: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_SetHeight(self.handle, height) }));
        Ok(self)
    }

    /// Returns the width of the line used to stroke a path.
    ///
    /// The default value is `1`.
    pub fn line_width(&self) -> f32 {
        unsafe { haru::HPDF_Page_GetLineWidth(self.handle) }
    }

    /// Sets the width of the line used to stroke a path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_line_width(&mut self, line_width: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_SetLineWidth(self.handle, line_width)
        }));
        Ok(self)
    }

    /// Returns the line cap used by paths.
    ///
    /// The default value is `LineCap::Butt`.
    pub fn line_cap(&self) -> LineCap {
        types::line_cap_from_int(unsafe { haru::HPDF_Page_GetLineCap(self.handle) })
    }

    /// Sets the line cap used by paths.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_line_cap(&mut self, line_cap: LineCap) -> Result<&mut Self, Error> {
        let line_cap = types::line_cap_as_int(line_cap);
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_SetLineCap(self.handle, line_cap) }));
        Ok(self)
    }

    /// Returns the join style used by paths.
    ///
    /// The default value is `LineJoin::Miter`.
    pub fn line_join(&self) -> LineJoin {
        types::line_join_from_int(unsafe { haru::HPDF_Page_GetLineJoin(self.handle) })
    }

    /// Sets the join style used by paths.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_line_join(&mut self, line_join: LineJoin) -> Result<&mut Self, Error> {
        let line_join = types::line_join_as_int(line_join);
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_SetLineJoin(self.handle, line_join) }));
        Ok(self)
    }

    /// Returns the miter limit used at the corners of a path stroke.
    ///
    /// The default value is `10.0`.
    pub fn miter_limit(&self) -> f32 {
        unsafe { haru::HPDF_Page_GetMiterLimit(self.handle) }
    }

    /// Sets the miter limit used at the corners of a path stroke.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_miter_limit(&mut self, miter_limit: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_SetMiterLimit(self.handle, miter_limit)
        }));
        Ok(self)
    }

    /// Returns the current dash pattern and its phase `(dash_pattern, phase)`.
    ///
    /// The default value is `([], 0)`
    pub fn dash(&self) -> (Box<[u16]>, u32) {
        let dash = unsafe { haru::HPDF_Page_GetDash(self.handle) };
        let slice = &dash.ptn[..dash.num_ptn as usize];
        (slice.to_vec().into_boxed_slice(), dash.phase)
    }

    /// Sets the dash pattern for lines in the page. The given phase sets the unit offset at which
    /// the dash pattern will start when drawn.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    ///
    /// # Examples
    ///
    /// ```norun
    /// // Dash (-) represents solid, dot (.) represents gap.
    ///
    /// // ---...
    /// page.set_dash(&[3], 0);
    ///
    /// // -----....---..
    /// page.set_dash(&[5, 4, 3, 2], 0);
    ///
    /// // ---....---..
    /// page.set_dash(&[5, 4, 3, 2], 2);
    ///
    /// // Completely solid (default):
    /// page.set_dash(&[], 0);
    ///
    /// // Error, odd pattern lengths greater than 1 disallowed:
    /// page.set_dash(&[1, 2, 3], 0);
    ///
    /// // Error, pattern lengths greater than 8 disallowed:
    /// page.set_dash(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 0);
    /// ```
    pub fn set_dash(&mut self, dash_pattern: &[u16], phase: u32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_SetDash(
            self.handle,
            dash_pattern.as_ptr(),
            dash_pattern.len() as u32,
            phase)
        }));
        Ok(self)
    }

    /// Returns the flatness tolerance, which is "the maximum permitted distance in device pixels
    /// between the mathematically correct path and an approximation constructed from straight line
    /// segments".
    ///
    /// The default value is `1.0`.
    pub fn flatness(&self) -> f32 {
        unsafe { haru::HPDF_Page_GetFlat(self.handle) }
    }

    /// Sets the flatness tolerance, which is "the maximum permitted distance in device pixels
    /// between the mathematically correct path and an approximation constructed from straight line
    /// segments".
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_flatness(&mut self, flatness: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_SetFlat(self.handle, flatness) }));
        Ok(self)
    }

    /// Returns the current value of the stroke color.
    ///
    /// This value is only valid if the current color space is set to gray (e.g. by calling
    /// `set_gray_stroke` prior to this method call).
    ///
    /// The default value is `0.0`.
    pub fn gray_stroke(&self) -> f32 {
        unsafe { haru::HPDF_Page_GetGrayStroke(self.handle) }
    }

    /// Sets the current color space to gray and sets the value of the stroke color.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_gray_stroke(&mut self, gray_stroke: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_SetGrayStroke(self.handle, gray_stroke)
        }));
        Ok(self)
    }

    /// Returns the current value of the fill color.
    ///
    /// This value is only valid if the current color space is set to gray (e.g. by calling
    /// `set_gray_fill` prior to this method call).
    ///
    /// The range of the returned floating-point value is between `0.0` and `1.0`
    ///
    /// The default value is `0.0`.
    pub fn gray_fill(&self) -> f32 {
        unsafe { haru::HPDF_Page_GetGrayFill(self.handle) }
    }

    /// Sets the current color space to gray and sets the value of the fill color.
    ///
    /// The floating-point value must be between `0.0` and `1.0`
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_gray_fill(&mut self, gray_fill: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_SetGrayFill(self.handle, gray_fill) }));
        Ok(self)
    }

    /// Returns the current value of the stroke color.
    ///
    /// This value is only valid if the current color space is set to RGB (e.g. by calling
    /// `set_rgb_stroke` prior to this method call).
    ///
    /// The range of each of the returned floating-point value is between `0.0` and `1.0`
    ///
    /// The default value is `(0.0, 0.0, 0.0)`.
    pub fn rgb_stroke(&self) -> (f32, f32, f32) {
        let color = unsafe { haru::HPDF_Page_GetRGBStroke(self.handle) };
        (color.r, color.g, color.b)
    }

    /// Sets the current color space to RGB and sets the value of the stroke color.
    ///
    /// Each floating-point value must be between `0.0` and `1.0`
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_rgb_stroke(&mut self, r: f32, g: f32, b: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_SetRGBStroke(self.handle, r, g, b) }));
        Ok(self)
    }

    /// Returns the current value of the fill color.
    ///
    /// This value is only valid if the current color space is set to RGB (e.g. by calling
    /// `set_rgb_fill` prior to this method call).
    ///
    /// The range of each of the returned floating-point value is between `0.0` and `1.0`
    ///
    /// The default value is `(0.0, 0.0, 0.0)`.
    pub fn rgb_fill(&self) -> (f32, f32, f32) {
        let color = unsafe { haru::HPDF_Page_GetRGBFill(self.handle) };
        (color.r, color.g, color.b)
    }

    /// Sets the current color space to RGB and sets the value of the fill color.
    ///
    /// Each floating-point value must be between `0.0` and `1.0`
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_rgb_fill(&mut self, r: f32, g: f32, b: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_SetRGBFill(self.handle, r, g, b) }));
        Ok(self)
    }

    /// Returns the current value of the stroke color.
    ///
    /// This value is only valid if the current color space is set to CMYK (e.g. by calling
    /// `set_cmyk_stroke` prior to this method call).
    ///
    /// The range of each of the returned floating-point value is between `0.0` and `1.0`
    ///
    /// The default value is `(0.0, 0.0, 0.0, 0.0)`.
    pub fn cmyk_stroke(&self) -> (f32, f32, f32, f32) {
        let color = unsafe { haru::HPDF_Page_GetCMYKStroke(self.handle) };
        (color.c, color.m, color.y, color.k)
    }

    /// Sets the current color space to CMYK and sets the value of the stroke color.
    ///
    /// Each floating-point value must be between `0.0` and `1.0`
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_cmyk_stroke(&mut self, c: f32, m: f32, y: f32, k: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_SetCMYKStroke(self.handle, c, m, y, k)
        }));
        Ok(self)
    }

    /// Returns the current value of the fill color.
    ///
    /// This value is only valid if the current color space is set to CMYK (e.g. by calling
    /// `set_cmyk_fill` prior to this method call).
    ///
    /// The range of each of the returned floating-point value is between `0.0` and `1.0`
    ///
    /// The default value is `(0.0, 0.0, 0.0, 0.0)`.
    pub fn cmyk_fill(&self) -> (f32, f32, f32, f32) {
        let color = unsafe { haru::HPDF_Page_GetCMYKFill(self.handle) };
        (color.c, color.m, color.y, color.k)
    }

    /// Sets the current color space to CMYK and sets the value of the fill color.
    ///
    /// Each floating-point value must be between `0.0` and `1.0`
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_cmyk_fill(&mut self, c: f32, m: f32, y: f32, k: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_SetCMYKFill(self.handle, c, m, y, k)
        }));
        Ok(self)
    }

    /// Returns the active stroke color space.
    ///
    /// Defaults to `ColorSpace::DeviceGray`.
    pub fn stroke_color_space(&self) -> Result<ColorSpace, Error> {
        match unsafe { haru::HPDF_Page_GetStrokingColorSpace(self.handle) } {
            haru::Enum__HPDF_ColorSpace::HPDF_CS_EOF => Err(Error::InvalidPage),
            color_space => Ok(types::color_space_from_int(color_space)),
        }
    }

    /// Returns the active fill color space.
    ///
    /// Defaults to `ColorSpace::DeviceGray`.
    pub fn fill_color_space(&self) -> Result<ColorSpace, Error> {
        match unsafe { haru::HPDF_Page_GetFillingColorSpace(self.handle) } {
            haru::Enum__HPDF_ColorSpace::HPDF_CS_EOF => Err(Error::InvalidPage),
            color_space => Ok(types::color_space_from_int(color_space)),
        }
    }

    /// Sets the starting point for the next path to the specified point.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn move_to(&mut self, point: Point) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_MoveTo(self.handle, point.x, point.y)
        }));
        Ok(self)
    }

    /// Appends a path from the current point to the specified point.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn line_to(&mut self, end: Point) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_LineTo(self.handle, end.x, end.y) }));
        Ok(self)
    }

    /// Appends a rectangle to the current path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn rectangle(&mut self, lower_left: Point, size: Size) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_Rectangle(self.handle, lower_left.x, lower_left.y, size.width,
                                      size.height)
        }));
        Ok(self)
    }

    /// Appends a circle to the current path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn circle(&mut self, center: Point, radius: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_Circle(self.handle, center.x, center.y, radius)
        }));
        Ok(self)
    }

    /// Appends a circle arc to the current path. Angles are given in degrees, with 0 degrees being
    /// vertical, upward, from the `center` position.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn arc(&mut self, center: Point, radius: f32, angle_begin: f32,
               angle_end: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_Arc(self.handle, center.x, center.y, radius, angle_begin, angle_end)
        }));
        Ok(self)
    }

    /// Append a cubic Bézier curve to the current path. The curve extends from the current point
    /// to the point `p3`, using `p1` and `p2` as the Bézier control points. The new current point
    /// is `p3`.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn curve_to(&mut self, p1: Point, p2: Point, p3: Point) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_CurveTo(self.handle, p1.x, p1.y, p2.x, p2.y, p3.x, p3.y)
        }));
        Ok(self)
    }

    /// Append a cubic Bézier curve to the current path. The curve extends from the current point
    /// to the point `p3`, using the current point and `p2` as the Bézier control points. The new
    /// current point is `p3`.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn curve_to_2(&mut self, p2: Point, p3: Point) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_CurveTo2(self.handle, p2.x, p2.y, p3.x, p3.y)
        }));
        Ok(self)
    }

    /// Append a cubic Bézier curve to the current path. The curve extends from the current point
    /// to the point `p3`, using `p1` and `p3` as the Bézier control points. The new current point
    /// is `p3`.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn curve_to_3(&mut self, p1: Point, p3: Point) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_CurveTo3(self.handle, p1.x, p1.y, p3.x, p3.y)
        }));
        Ok(self)
    }

    /// Paints the current path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn stroke(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_Stroke(self.handle) }));
        Ok(self)
    }

    /// Fills the current path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn fill(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_Fill(self.handle) }));
        Ok(self)
    }

    /// Fills the current path using the even-odd rule.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn eo_fill(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_Eofill(self.handle) }));
        Ok(self)
    }

    /// Fills and strokes the current path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn fill_stroke(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_FillStroke(self.handle) }));
        Ok(self)
    }

    /// Fills the current path using the even-odd rule, then strokes the path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn eo_fill_stroke(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_EofillStroke(self.handle) }));
        Ok(self)
    }

    /// Appends a straight line from the current point to the start point of sub path. The current
    /// point is moved to the start point of sub path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn close_path(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_ClosePath(self.handle) }));
        Ok(self)
    }

    /// Closes the current path, then paints it with a stroke.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn close_path_stroke(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_ClosePathStroke(self.handle) }));
        Ok(self)
    }

    /// Closes the current path, fills the current path using the even-odd rule, then paints the
    /// path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn close_path_eo_fill_stroke(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_ClosePathEofillStroke(self.handle) }));
        Ok(self)
    }

    /// Closes the current path, fills the current path using the non-zero winding number rule,
    /// then paints the path.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn close_path_fill_stroke(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_ClosePathFillStroke(self.handle) }));
        Ok(self)
    }

    /// Returns the current position for path painting.
    ///
    /// This method returns `(0.0, 0.0)` if the page is not currently in path drawing mode.
    pub fn position(&self) -> Point {
        let point = unsafe { haru::HPDF_Page_GetCurrentPos(self.handle) };
        Point::new(point.x, point.y)
    }

    /// Ends the current path without filling or stroking.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn end_path(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_EndPath(self.handle) }));
        Ok(self)
    }

    /// Returns the active font, if any.
    pub fn font(&self) -> Option<Font> {
        let handle = unsafe { haru::HPDF_Page_GetCurrentFont(self.handle) };
        if handle == ptr::null_mut() {
            None
        } else {
            Some(font::new(handle, self.doc.clone()))
        }
    }

    /// Returns the active font size, if any.
    pub fn font_size(&self) -> Option<f32> {
        match unsafe { haru::HPDF_Page_GetCurrentFontSize(self.handle) } {
            0.0 => None,
            size => Some(size),
        }
    }

    /// Returns the current position for path painting.
    ///
    /// This method returns `(0.0, 0.0)` if the page is not currently in text mode (see
    /// `Page::begin_text`).
    pub fn text_position(&self) -> Point {
        let point = unsafe { haru::HPDF_Page_GetCurrentTextPos(self.handle) };
        Point::new(point.x, point.y)
    }

    /// Moves the text position to the next line.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn move_to_next_line(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_MoveToNextLine(self.handle) }));
        Ok(self)
    }

    /// Sets the active font and its size in points.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_font_and_size(&mut self, font: &Font, size: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_SetFontAndSize(self.handle, font::get_handle(font), size)
        }));
        Ok(self)
    }

    /// Sets the text leading (line spacing) for text drawing.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn set_text_leading(&mut self, leading: f32) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_SetTextLeading(self.handle, leading)
        }));
        Ok(self)
    }

    /// Begins text object graphics mode with the text position `(0.0, 0.0)`.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn begin_text(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_BeginText(self.handle) }));
        Ok(self)
    }

    /// Ends text output graphics mode.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn end_text(&mut self) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe { haru::HPDF_Page_EndText(self.handle) }));
        Ok(self)
    }

    /// Sets the text position to the specified point and prints the text.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn text_out(&mut self, text: &str, baseline_left: Point) -> Result<&mut Self, Error> {
        let text = try!(CString::new(text));
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_TextOut(self.handle, baseline_left.x, baseline_left.y, text.as_ptr())
        }));
        Ok(self)
    }

    /// Offsets the point at which the next text will be drawn. If the current text position is
    /// `(x, y)`, the new text position will be `(x + x', y + y')`.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn offset_text_position(&mut self, point: Point) -> Result<&mut Self, Error> {
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_MoveTextPos(self.handle, point.x, point.y)
        }));
        Ok(self)
    }

    /// Prints the text at the current position on the page.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn show_text(&mut self, text: &str) -> Result<&mut Self, Error> {
        let text = try!(CString::new(text));
        try!(self.doc.check_error(unsafe {
            haru::HPDF_Page_ShowText(self.handle, text.as_ptr())
        }));
        Ok(self)
    }

    /// Prints the text inside the specified region using the specified alignment.
    ///
    /// The text will be silently clipped if it does not entirely fit in the region.
    ///
    /// This operation conveniently returns a reference to itself for chaining commands.
    pub fn text_rect(&mut self, text: &str, lower_left: Point, size: Size,
                     alignment: TextAlignment) -> Result<&mut Self, Error> {
        let result = unsafe {
            haru::HPDF_Page_TextRect(self.handle, lower_left.x, lower_left.y + size.height,
                                     lower_left.x + size.width, lower_left.y,
                                     text.as_ptr() as *const i8,
                                     types::text_alignment_as_int(alignment), ptr::null_mut())
        };

        match self.doc.check_error(result) {
            Ok(()) | Err(Error::PageInsufficientSpace) => Ok(self),
            Err(err) => Err(err)
        }
    }
}

/// Creates a new `Page` from a raw libharu page handle and its owner document.
#[inline]
pub fn new(page: haru::HPDF_Page, doc: Rc<DocumentHandle>) -> Page {
    Page { handle: page, doc: doc }
}

/// Extracts the libharu handle from the given `Page`.
#[inline]
pub fn get_handle(page: &Page) -> haru::HPDF_Page {
    page.handle
}

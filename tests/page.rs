extern crate libharu;

mod util;

use libharu::*;
use std::fs::File;
use util::*;

#[test]
fn width() {
    with_page(|document, page| {
        assert_eq!(page.width(), 595.0);
        page.set_width(1000.0).unwrap();
        assert_eq!(page.width(), 1000.0);
        assert_pdf("page_width", document);
    });
}

#[test]
fn height() {
    with_page(|document, page| {
        assert_eq!(page.height(), 841.0);
        page.set_height(500.0).unwrap();
        assert_eq!(page.height(), 500.0);
        assert_pdf("page_height", document);
    });
}

#[test]
fn stroke() {
    with_page(|document, page| {
        draw_angle_stroke(page);
        assert_pdf("page_stroke", document);
    });
}

#[test]
fn fill() {
    with_page(|document, page| {
        draw_overlapping_shape(page);
        page.fill().unwrap();
        assert_pdf("page_fill", document);
    });
}

#[test]
fn eo_fill() {
    with_page(|document, page| {
        draw_overlapping_shape(page);
        page.eo_fill().unwrap();
        assert_pdf("page_eo_fill", document);
    });
}

#[test]
fn fill_stroke() {
    with_page(|document, page| {
        draw_rectangle_fill_stroke(page);
        assert_pdf("page_fill_stroke", document);
    });
}

#[test]
fn eo_fill_stroke() {
    with_page(|document, page| {
        page.set_gray_fill(0.5).unwrap();
        draw_overlapping_shape(page);
        page.eo_fill_stroke().unwrap();
        assert_pdf("page_eo_fill_stroke", document);
    });
}

#[test]
fn rectangle() {
    with_page(|document, page| {
        page.rectangle(Point::new(10.0, 10.0), Size::new(40.0, 30.0)).unwrap();
        page.fill().unwrap();
        assert_pdf("page_rectangle", document);
    });
}

#[test]
fn circle() {
    with_page(|document, page| {
        page.circle(Point::new(30.0, 30.0), 20.0).unwrap();
        page.fill().unwrap();
        assert_pdf("page_circle", document);
    });
}

#[test]
fn arc() {
    with_page(|document, page| {
        page.arc(Point::new(30.0, 30.0), 20.0, 30.0, 150.0).unwrap();
        page.stroke().unwrap();
        assert_pdf("page_arc", document);
    });
}

#[test]
fn curve_to() {
    with_page(|document, page| {
        page.move_to(Point::new(20.0, 20.0)).unwrap();
        let (a, b, c) = (Point::new(40.0, 40.0), Point::new(55.0, 35.0), Point::new(50.0, 10.0));
        page.curve_to(a, b, c).unwrap();
        page.stroke().unwrap();
        assert_pdf("page_curve_to", document);
    });
}

#[test]
fn curve_to_2() {
    with_page(|document, page| {
        page.move_to(Point::new(10.0, 10.0)).unwrap();
        page.curve_to_2(Point::new(40.0, 35.0), Point::new(60.0, 20.0)).unwrap();
        page.stroke().unwrap();
        assert_pdf("page_curve_to_2", document);
    });
}

#[test]
fn curve_to_3() {
    with_page(|document, page| {
        page.move_to(Point::new(10.0, 10.0)).unwrap();
        page.curve_to_3(Point::new(40.0, 35.0), Point::new(60.0, 20.0)).unwrap();
        page.stroke().unwrap();
        assert_pdf("page_curve_to_3", document);
    });
}

#[test]
fn line_width() {
    with_page(|document, page| {
        assert_eq!(page.line_width(), 1.0);
        page.set_line_width(5.0).unwrap();
        assert_eq!(page.line_width(), 5.0);
        draw_angle_stroke(page);
        assert_pdf("page_line_width", document);
    });
}

#[test]
fn line_cap() {
    with_page(|document, page| {
        // Don't worry, line cap. **Everyone** is butt by default.
        assert_eq!(page.line_cap(), LineCap::Butt);
        // Give the line a bit of width to see the round cap.
        page.set_line_width(5.0).unwrap();
        page.set_line_cap(LineCap::Round).unwrap();
        assert_eq!(page.line_cap(), LineCap::Round);
        draw_angle_stroke(page);
        assert_pdf("page_line_cap", document);
    });
}

#[test]
fn line_join() {
    with_page(|document, page| {
        assert_eq!(page.line_join(), LineJoin::Miter);
        // Give the line a bit of width to see the bevelled joint.
        page.set_line_width(5.0).unwrap();
        page.set_line_join(LineJoin::Bevel).unwrap();
        assert_eq!(page.line_join(), LineJoin::Bevel);
        draw_angle_stroke(page);
        assert_pdf("page_line_join", document);
    });
}

#[test]
fn miter_limit() {
    with_page(|document, page| {
        assert_eq!(page.miter_limit(), 10.0);
        page.set_miter_limit(50.0).unwrap();
        assert_eq!(page.miter_limit(), 50.0);
        draw_angle_stroke(page);
        assert_pdf("page_miter_limit", document);
    });
}

#[test]
fn dash() {
    with_page(|document, page| {
        assert_eq!(page.dash(), (Box::new([]) as Box<[_]>, 0));
        assert!(page.set_dash(&[1, 2, 3], 0).is_err());
        assert!(page.set_dash(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 0).is_err());
        page.set_dash(&[30, 10], 20).unwrap();
        assert_eq!(page.dash(), (Box::new([30u16, 10]) as Box<[_]>, 20));
        draw_angle_stroke(page);
        assert_pdf("page_dash", document);
    });
}

#[test]
fn gray_stroke() {
    with_page(|document, page| {
        assert_eq!(page.gray_stroke(), 0.0);
        page.set_gray_stroke(0.5).unwrap();
        assert_eq!(page.stroke_color_space().unwrap(), ColorSpace::DeviceGray);
        assert_eq!(page.gray_stroke(), 0.5);
        draw_rectangle_fill_stroke(page);
        assert_pdf("page_gray_stroke", document);
    });
}

#[test]
fn gray_fill() {
    with_page(|document, page| {
        assert_eq!(page.gray_fill(), 0.0);
        page.set_gray_fill(0.5).unwrap();
        assert_eq!(page.fill_color_space().unwrap(), ColorSpace::DeviceGray);
        assert_eq!(page.gray_fill(), 0.5);
        draw_rectangle_fill_stroke(page);
        assert_pdf("page_gray_fill", document);
    });
}

#[test]
fn rgb_stroke() {
    with_page(|document, page| {
        assert_eq!(page.rgb_stroke(), (0.0, 0.0, 0.0));
        page.set_rgb_stroke(1.0, 0.2, 0.4).unwrap();
        assert_eq!(page.stroke_color_space().unwrap(), ColorSpace::DeviceRgb);
        assert_eq!(page.rgb_stroke(), (1.0, 0.2, 0.4));
        draw_rectangle_fill_stroke(page);
        assert_pdf("page_rgb_stroke", document);
    });
}

#[test]
fn rgb_fill() {
    with_page(|document, page| {
        assert_eq!(page.rgb_fill(), (0.0, 0.0, 0.0));
        page.set_rgb_fill(1.0, 0.2, 0.4).unwrap();
        assert_eq!(page.fill_color_space().unwrap(), ColorSpace::DeviceRgb);
        assert_eq!(page.rgb_fill(), (1.0, 0.2, 0.4));
        draw_rectangle_fill_stroke(page);
        assert_pdf("page_rgb_fill", document);
    });
}

#[test]
fn cmyk_stroke() {
    with_page(|document, page| {
        assert_eq!(page.cmyk_stroke(), (0.0, 0.0, 0.0, 0.0));
        page.set_cmyk_stroke(0.0, 0.77, 0.85, 0.17).unwrap();
        assert_eq!(page.stroke_color_space().unwrap(), ColorSpace::DeviceCmyk);
        assert_eq!(page.cmyk_stroke(), (0.0, 0.77, 0.85, 0.17));
        draw_rectangle_fill_stroke(page);
        assert_pdf("page_cmyk_stroke", document);
    });
}

#[test]
fn cmyk_fill() {
    with_page(|document, page| {
        assert_eq!(page.cmyk_fill(), (0.0, 0.0, 0.0, 0.0));
        page.set_cmyk_fill(0.0, 0.77, 0.85, 0.17).unwrap();
        assert_eq!(page.fill_color_space().unwrap(), ColorSpace::DeviceCmyk);
        assert_eq!(page.cmyk_fill(), (0.0, 0.77, 0.85, 0.17));
        draw_rectangle_fill_stroke(page);
        assert_pdf("page_cmyk_fill", document);
    });
}

#[test]
fn end_path() {
    with_page(|document, page| {
        // Move with a diagonal path without drawing.
        page.move_to(Point::new(10.0, 10.0)).unwrap();
        page.line_to(Point::new(50.0, 50.0)).unwrap();
        page.end_path().unwrap();
        // Move with a vertical path with stroking.
        page.move_to(Point::new(50.0, 50.0)).unwrap();
        page.line_to(Point::new(50.0, 10.0)).unwrap();
        page.stroke().unwrap();
        assert_pdf("page_end_path", document);
    });
}

#[test]
fn close_path() {
    with_page(|document, page| {
        page.move_to(Point::new(10.0, 10.0)).unwrap();
        page.line_to(Point::new(50.0, 50.0)).unwrap();
        page.line_to(Point::new(50.0, 10.0)).unwrap();
        page.close_path().unwrap();
        page.stroke().unwrap();
        assert_pdf("page_close_path", document);
    });
}

#[test]
fn close_path_stroke() {
    with_page(|document, page| {
        page.move_to(Point::new(10.0, 10.0)).unwrap();
        page.line_to(Point::new(50.0, 50.0)).unwrap();
        page.line_to(Point::new(50.0, 10.0)).unwrap();
        page.close_path_stroke().unwrap();
        assert_pdf("page_close_path_stroke", document);
    });
}

#[test]
fn close_path_fill_stroke() {
    with_page(|document, page| {
        page.set_gray_fill(0.5).unwrap();
        draw_overlapping_shape(page);
        page.close_path_fill_stroke().unwrap();
        assert_pdf("page_close_path_fill_stroke", document);
    });
}

#[test]
fn close_path_eo_fill_stroke() {
    with_page(|document, page| {
        page.set_gray_fill(0.5).unwrap();
        draw_overlapping_shape(page);
        page.close_path_eo_fill_stroke().unwrap();
        assert_pdf("page_close_path_eo_fill_stroke", document);
    });
}

#[test]
fn stroke_color_space() {
    with_page(|_, page| assert_eq!(page.stroke_color_space().unwrap(), ColorSpace::DeviceGray));
}

#[test]
fn fill_color_space() {
    with_page(|_, page| assert_eq!(page.fill_color_space().unwrap(), ColorSpace::DeviceGray));
}

#[test]
fn text_out() {
    with_page(|document, page| {
        page.set_font_and_size(&load_font(document), 20.0).unwrap()
            .begin_text().unwrap()
            .text_out("Test text!", Point::new(10.0, 10.0)).unwrap()
            .end_text().unwrap();
        assert_pdf("page_text_out", document);
    });
}

#[test]
fn show_text() {
    with_page(|document, page| {
        page.set_font_and_size(&load_font(document), 20.0).unwrap()
            .begin_text().unwrap()
            .offset_text_position(Point::new(20.0, 10.0)).unwrap()
            .show_text("Test text!").unwrap()
            .end_text().unwrap();
        assert_pdf("page_show_text", document);
    });
}

#[test]
fn text_rect() {
    with_page(|document, page| {
        let point = Point::new(10.0, 10.0);
        let size = Size::new(100.0, 100.0);
        page.set_font_and_size(&load_font(document), 8.0).unwrap()
            .begin_text().unwrap()
            .text_rect(LOREM_IPSUM, point, size, TextAlignment::Center).unwrap()
            .end_text().unwrap();
        assert_pdf("page_text_rect", document);
    });
}

#[test]
fn set_font_and_size() {
    with_page(|document, page| {
        assert!(page.font().is_none());
        assert!(page.font_size().is_none());
        page.set_font_and_size(&load_font(document), 20.0).unwrap();
        assert!(page.font().is_some());
        assert_eq!(page.font_size(), Some(20.0));
    });
}

#[test]
fn position() {
    with_page(|_, page| {
        assert_eq!(page.position(), Point::new(0.0, 0.0));
        page.move_to(Point::new(10.0, 10.0)).unwrap();
        assert_eq!(page.position(), Point::new(10.0, 10.0));
    });
}

#[test]
fn text_position() {
    with_page(|document, page| {
        assert_eq!(page.text_position(), Point::new(0.0, 0.0));
        page.set_font_and_size(&load_font(document), 20.0).unwrap().begin_text().unwrap();
        assert_eq!(page.text_position(), Point::new(0.0, 0.0));
        page.text_out("Test text!", Point::new(10.0, 10.0)).unwrap();
        assert_eq!(page.text_position(), Point::new(119.0, 10.0));
    });
}

#[test]
fn move_to_next_line() {
    with_page(|document, page| {
        page.set_text_leading(20.0).unwrap();
        page.set_font_and_size(&load_font(document), 20.0).unwrap();
        page.begin_text().unwrap();
        page.show_text("Test text!").unwrap();
        assert_eq!(page.text_position(), Point::new(109.0, 0.0));
        page.move_to_next_line().unwrap();
        assert_eq!(page.text_position(), Point::new(0.0, -20.0));
    });
}

#[test]
fn offset_text_position() {
    with_page(|_, page| {
        page.begin_text().unwrap();
        assert_eq!(page.text_position(), Point::new(0.0, 0.0));
        page.offset_text_position(Point::new(20.0, 10.0)).unwrap();
        assert_eq!(page.text_position(), Point::new(20.0, 10.0));
        page.offset_text_position(Point::new(10.0, 25.0)).unwrap();
        assert_eq!(page.text_position(), Point::new(30.0, 35.0));
    });
}

#[inline]
fn with_page<F: FnOnce(&mut Document, &mut Page)>(f: F) {
    let mut document = Document::new().unwrap();
    let mut page = document.add_page().unwrap();
    f(&mut document, &mut page);
}

// Draws a small, stroked acute angle near the bottom-left corner of the page.
#[inline]
fn draw_angle_stroke(page: &mut Page) {
    page.move_to(Point::new(10.0, 10.0)).unwrap();
    page.line_to(Point::new(50.0, 50.0)).unwrap();
    page.line_to(Point::new(50.0, 10.0)).unwrap();
    page.stroke().unwrap();
}

// Draws a small, stroked and filled rectangle near the bottom-left corner of the page.
#[inline]
fn draw_rectangle_fill_stroke(page: &mut Page) {
    page.rectangle(Point::new(10.0, 10.0), Size::new(40.0, 30.0)).unwrap();
    page.fill_stroke().unwrap();
}

// Draws a small, complex shape near the bottom-left corner of the page. This shape has overlapping
// areas for testing fill mechanisms.
//
// No stroking or filling is done.
#[inline]
fn draw_overlapping_shape(page: &mut Page) {
    page.move_to(Point::new(10.0, 10.0)).unwrap();
    page.line_to(Point::new(50.0, 50.0)).unwrap();
    page.line_to(Point::new(50.0, 10.0)).unwrap();
    page.line_to(Point::new(10.0, 50.0)).unwrap();
    page.line_to(Point::new(30.0, 50.0)).unwrap();
    page.line_to(Point::new(50.0, 30.0)).unwrap();
}

#[inline]
fn load_font(document: &mut Document) -> Font {
    let file = File::open(fixture_path("ttf/gohufont-11.ttf")).unwrap();
    document.load_ttf_font(file).unwrap()
}

static LOREM_IPSUM: &'static str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Ut posuere nulla vel sem lacinia facilisis sit amet eget nunc.

Integer laoreet, arcu vitae aliquet dignissim, nisl enim lobortis nulla, ac hendrerit neque ex \
eget sapien. Proin at ante arcu.

Ut tristique, nunc et lacinia volutpat, dui augue porta felis, id gravida quam libero vitae \
velit. Sed consequat augue quis turpis tincidunt, eu convallis leo fringilla. Integer eleifend \
ipsum nec tempus scelerisque. Sed quis fermentum tortor, aliquet sollicitudin nulla.";

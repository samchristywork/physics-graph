use rusttype::{point, Font, Scale};

use crate::vector::Vector;

#[must_use]
pub fn move_to(v: Vector) -> String {
    let a = v.x;
    let b = v.y;

    format!("M{a},{b} ")
}

#[must_use]
pub fn start(x: f32, y: f32, width: f32, height: f32) -> String {
    format!("<svg viewBox=\"{x} {y} {width} {height}\" xmlns=\"http://www.w3.org/2000/svg\">")
}

#[must_use]
pub fn draw_line(start: Vector, end: Vector) -> String {
    let x1 = start.x;
    let y1 = start.y;
    let x2 = end.x;
    let y2 = end.y;

    format!(
        "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" \
        style=\"stroke:black;stroke-width:0.002\" />"
    )
}

#[must_use]
pub fn line_to(v: Vector) -> String {
    let x = v.x;
    let y = v.y;

    format!("L{x}, {y}")
}

#[must_use]
pub fn draw_box(x: f32, y: f32, width: f32, height: f32, stroke: f32, color: &str) -> String {
    format!(
        "<path d=\"M{x},{y} l0,{height} l{width},0 l0,-{height} z\" \
        fill=\"none\" stroke=\"{color}\" stroke-width=\"{stroke}\"/>"
    )
}

#[must_use]
pub fn draw_label(x: f32, y: f32, font_size: f32, font_color: &str, label: &str) -> String {
    format!(
        "<text x=\"{x}\" y=\"{y}\" text-anchor=\"middle\" font-size=\"{font_size}\" \
        fill=\"{font_color}\">{label}</text>"
    )
}

#[must_use]
pub fn draw_label_multiline(
    x: f32,
    y: f32,
    font_size: f32,
    font_color: &str,
    label: &str,
) -> String {
    let mut svg_text = String::new();
    let mut svg_background = String::new();

    let line_height = font_size + 0.01;
    let mut label_width = 0.0;
    let mut label_height = 0.0;

    let y =
        y + font_size - line_height * label.split("\\n").collect::<Vec<&str>>().len() as f32 / 2.0;

    for line in label.split("\\n") {
        let font_data = include_bytes!("../LiberationSans-Regular.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

        let scale = Scale::uniform(32.0);

        let glyphs: Vec<_> = font.layout(line, scale, point(0.0, 0.0)).collect();
        let text_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x) as f32 / 600.0 + 0.02
        };

        if label_width < text_width {
            label_width = text_width;
        }

        let y = y + label_height;
        svg_text += format!(
            "<text x=\"{x}\" y=\"{y}\" text-anchor=\"middle\" font-size=\"{font_size}\" \
            fill=\"{font_color}\">{line}</text>"
        )
        .as_str();

        label_height += line_height;
    }

    let label_background_x = x - label_width / 2.0;
    let label_background_y = y - line_height + 0.015;

    svg_background += format!(
        "<rect x=\"{label_background_x}\" y=\"{label_background_y}\" \
        width=\"{label_width}\" height=\"{label_height}\" \
            style=\"fill:white;stroke-width:0.001;stroke:black\"></rect>"
    )
    .as_str();

    svg_background + svg_text.as_str()
}

#[must_use]
pub fn draw_box_with_label(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    stroke: f32,
    font_size: f32,
    color: &str,
    font_color: &str,
    label: &str,
) -> String {
    format!(
        "<path d=\"M{x},{y} l0,{height} l{width},0 l0,-{height} z\" \
        fill=\"none\" stroke=\"{color}\" stroke-width=\"{stroke}\"/> \
        <text x=\"{x}\" y=\"{y}\" font-size=\"{font_size}\" fill=\"{font_color}\">{label}</text>"
    )
}

#[must_use]
pub fn end() -> String {
    "</svg>".to_string()
}

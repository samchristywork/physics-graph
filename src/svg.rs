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
pub fn draw_line(start: Vector, end: Vector, color: &str) -> String {
    let x1 = start.x;
    let y1 = start.y;
    let x2 = end.x;
    let y2 = end.y;

    format!(
        "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" \
        style=\"stroke:{color};stroke-width:0.004\" />"
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
    let mut y = y;
    let mut svg = String::new();

    for line in label.split("\\n") {
        svg += format!(
            "<text x=\"{x}\" y=\"{y}\" text-anchor=\"middle\" font-size=\"{font_size}\" \
            fill=\"{font_color}\">{line}</text>"
        )
        .as_str();
        y += font_size;
    }

    svg
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

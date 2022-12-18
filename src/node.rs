use crate::svg;

pub struct Node<'a> {
    pub x: f32,
    pub y: f32,
    pub name: &'a str,
    pub visited: bool,
}

impl Node<'_> {
    pub fn draw(&self) -> String {
        let text_color = "black";
        format!(
            "{}\n",
            svg::draw_label_multiline(self.x, self.y, 0.05, text_color, &self.name)
        )
    }
}

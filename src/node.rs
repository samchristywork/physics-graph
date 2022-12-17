use crate::svg;

pub struct Node<'a> {
    pub x: f32,
    pub y: f32,
    pub name: &'a str,
    pub visited: bool,
}

impl Node<'_> {
    pub fn draw(&self) -> String {
        let text_color = "red";
        format!(
            "{}\n",
            svg::draw_label(self.x, self.y, 0.1, text_color, &self.name)
        )
    }
}

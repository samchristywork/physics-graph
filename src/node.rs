use crate::svg;

pub struct Node<'a> {
    pub x: f32,
    pub y: f32,
    pub name: &'a str,
    pub color: String,
    pub visited: bool,
    pub visible: bool,
}

impl Node<'_> {
    pub fn draw(&self) -> String {
        let text_color = self.color.as_str();

        if self.visible == false {
            return "".to_string();
        }

        format!(
            "{}\n",
            svg::draw_label_multiline(self.x, self.y, 0.05, text_color, &self.name)
        )
    }
}

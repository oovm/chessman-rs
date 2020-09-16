use svg::{
    node::element::{Circle, Line, Rectangle, Text},
    Document,
};

pub struct SvgRender {
    pub grid_size: f32,
    pub board_white: String,
    pub board_black: String,
    pub path_color: String,
    pub path_width: f32,
}

impl Default for SvgRender {
    fn default() -> Self {
        Self {
            grid_size: 100.0,
            board_white: "#FFCF9E".to_string(),
            board_black: "#D18A47".to_string(),
            path_color: "#FF0000".to_string(),
            path_width: 6.0,
        }
    }
}

impl SvgRender {
    pub fn document(&self, width: f32, height: f32) -> Document {
        Document::new().set("viewBox", (0, 0, width * self.grid_size, height * self.grid_size))
    }
    pub fn grid_color(&self, x: isize, y: isize) -> &str {
        if (x + y) % 2 == 0 { self.board_white.as_str() } else { self.board_black.as_str() }
    }
    pub fn draw_square(&self, x: isize, y: isize) -> Rectangle {
        Rectangle::new()
            .set("x", x as f32 * self.grid_size)
            .set("y", y as f32 * self.grid_size)
            .set("width", self.grid_size)
            .set("height", self.grid_size)
            .set("fill", self.grid_color(x, y))
    }
    pub fn draw_path(&self, x1: isize, y1: isize, x2: isize, y2: isize) -> Line {
        Line::new()
            .set("x1", self.center_position(x1))
            .set("y1", self.center_position(y1))
            .set("x2", self.center_position(x2))
            .set("y2", self.center_position(y2))
            .set("stroke", self.path_color.as_str())
            .set("stroke-width", self.grid_size / 12.0)
    }
    pub fn draw_point(&self, x: isize, y: isize) -> Circle {
        Circle::new()
            .set("cx", self.center_position(x))
            .set("cy", self.center_position(y))
            .set("r", self.grid_size / 6.0)
            .set("fill", self.path_color.as_str())
    }

    pub fn draw_step(&self, x: isize, y: isize, step: usize) -> Text {
        Text::new()
            .set("x", self.center_position(x))
            .set("y", self.center_position(y) + 12.0)
            .set("text-anchor", "middle")
            .set("font-size", self.grid_size / 2.0)
            .set("fill", "#000000")
            .add(svg::node::Text::new(step.to_string()))
    }
    fn center_position(&self, i: isize) -> f32 {
        i as f32 * self.grid_size + self.grid_size / 2.0
    }
}

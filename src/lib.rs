use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct CanvasPainter {
    context: web_sys::CanvasRenderingContext2d
}

impl CanvasPainter {
    pub fn new() -> CanvasPainter {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("rainbow-unicorn-canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        CanvasPainter {
            context: context
        }
    }

    pub fn paint(&self, graphics_primitives: &Vec<GraphicsPrimitive>) {
        for gp in graphics_primitives.iter() {
            match gp {
                GraphicsPrimitive::Rectangle { data } => {
                    // need to handle rounded corners
                    self.context.rect(
                        data.left as f64,
                        data.top as f64,
                        data.width as f64,
                        data.height as f64
                    );
                    self.context.stroke();
                }
                GraphicsPrimitive::Line { data } => {
                    self.context.begin_path();
                    self.context.move_to(data.x1 as f64, data.y1 as f64);
                    self.context.line_to(data.x2 as f64, data.y2 as f64);
                    self.context.stroke();
                }
                GraphicsPrimitive::Ellipse { data } => {
                    self.context.begin_path();
                    self.context.ellipse(
                        data.cx as f64,
                        data.cy as f64,
                        data.rx as f64,
                        data.ry as f64,
                        0.0,
                        0.0,
                        2.0 * f64::consts::PI
                    );
                }
                GraphicsPrimitive::Text { data } => {
                    self.context.set_font(data.font);
                    self.context.fill_text(
                        data.text,
                        data.x as f64,
                        data.y as f64
                    );
                }
                GraphicsPrimitive::Polygon { points } => {
                    self.context.begin_path();
                    self.context.move_to(points[0].x as f64, points[0].y as f64);
                    
                    for p in points.iter() {
                        self.context.line_to(p.x as f64, p.y as f64);
                    }
                    self.context.line_to(points[0].x as f64, points[0].y as f64);

                    // need to switch on this later based on specified styles
                    self.context.stroke();

                    self.context.close_path();
                }
                GraphicsPrimitive::RuPath { data } => {
                    self.context.begin_path();
                    self.context.move_to(data.start_x as f64, data.start_y as f64);

                    for segment in data.segments.iter() {
                        self.draw_path_segment(&self.context, segment);
                    }

                    self.context.stroke();
                    self.context.close_path();
                }
            }
        }
    }

    fn draw_path_segment(&self, context: &web_sys::CanvasRenderingContext2d, segment: &PathSegment) {
        match segment {
            PathSegment::MoveTo { to_x, to_y } => {
                context.move_to(*to_x as f64, *to_y as f64);
            }
            PathSegment::LineTo { to_x, to_y } => {
                context.line_to(*to_x as f64, *to_y as f64);
            }
            PathSegment::BezierCurveTo {
                cp1_x,
                cp1_y,
                cp2_x,
                cp2_y,
                to_x,
                to_y
            } => {
                context.bezier_curve_to(
                    *cp1_x as f64,
                    *cp1_y as f64,
                    *cp2_x as f64,
                    *cp2_y as f64,
                    *to_x as f64,
                    *to_y as f64
                );
            }
            PathSegment::QuadraticCurveTo {
                cp_x,
                cp_y,
                to_x,
                to_y
            } => {
                context.quadratic_curve_to(
                    *cp_x as f64,
                    *cp_y as f64,
                    *to_x as f64,
                    *to_y as f64
                );
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GraphicsPrimitive<'a> {
    Rectangle { data: RectangleData },
    Line { data: LineData },
    Ellipse { data: EllipseData },
    Text { data: TextData<'a> },
    Polygon { points: Vec<Point> },
    RuPath { data: PathData }
}

#[derive(PartialEq, Debug)]
pub struct RectangleData {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
    pub rx: u32,
    pub ry: u32,
}

#[derive(PartialEq, Debug)]
pub struct LineData {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}

#[derive(PartialEq, Debug)]
pub struct EllipseData {
    pub cx: u32,
    pub cy: u32,
    pub rx: u32,
    pub ry: u32,
}

#[derive(PartialEq, Debug)]
pub struct TextData<'a> {
    pub x: u32,
    pub y: u32,
    pub text: &'a str,
    pub font: &'a str,
}

#[derive(PartialEq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(PartialEq, Debug)]
pub enum PathSegment {
    MoveTo { to_x: u32, to_y: u32 },
    LineTo { to_x: u32, to_y: u32 },
    BezierCurveTo { cp1_x: u32, cp1_y: u32, cp2_x: u32, cp2_y: u32, to_x: u32, to_y: u32 },
    QuadraticCurveTo { cp_x: u32, cp_y: u32, to_x: u32, to_y: u32 }
}

#[derive(PartialEq, Debug)]
pub struct PathData {
    pub start_x: u32,
    pub start_y: u32,
    pub segments: Vec<PathSegment>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_rectangle() {
        GraphicsPrimitive::Rectangle { data: RectangleData {
            left: 100,
            top: 50,
            width: 200,
            height: 75,
            rx: 0,
            ry: 5
        }};
    }

    #[test]
    fn make_ellipse() {
        GraphicsPrimitive::Ellipse { data: EllipseData {
            cx: 60,
            cy: 70,
            rx: 30,
            ry: 45
        }};
    }

    #[test]
    fn make_line() {
        GraphicsPrimitive::Line { data: LineData {
            x1: 1,
            y1: 2,
            x2: 11,
            y2: 12
        }};
    }

    #[test]
    fn make_text() {
        GraphicsPrimitive::Text { data: TextData {
            x: 900,
            y: 1000,
            text: "Hello",
            font: "Dialog"
        }};
    }

    #[test]
    fn make_polygon() {
        GraphicsPrimitive::Polygon { points: vec![
            Point { x: 4, y: 41 },
            Point { x: 5, y: 45 }
        ]};
    }

    #[test]
    fn make_path() {
        GraphicsPrimitive::RuPath { data: PathData {
            start_x: 10,
            start_y: 20,
            segments: vec![
                PathSegment::MoveTo { to_x: 30, to_y: 40 },
                PathSegment::LineTo { to_x: 50, to_y: 20 },
                PathSegment::BezierCurveTo {
                    cp1_x: 100,
                    cp1_y: 300,
                    cp2_x: 400,
                    cp2_y: 500,
                    to_x: 225,
                    to_y: 275
                },
                PathSegment::QuadraticCurveTo {
                    cp_x: 2100,
                    cp_y: 1800,
                    to_x: 1922,
                    to_y: 1918
                }
            ]
        }};
    }
}

use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

mod primitives;
use primitives::*;

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
                GraphicsPrimitive::Rectangle { data, styles } => {
                    CanvasPainter::draw_rect(&self.context, data, styles);
                }
                GraphicsPrimitive::Line { data, styles } => {
                    CanvasPainter::draw_line(&self.context, data, styles);
                }
                GraphicsPrimitive::Ellipse { data, styles } => {
                    CanvasPainter::draw_ellipse(&self.context, data, styles);
                }
                GraphicsPrimitive::Text { data, styles } => {
                    CanvasPainter::draw_text(&self.context, data, styles);
                }
                GraphicsPrimitive::Polygon { points, styles } => {
                    CanvasPainter::draw_polygon(&self.context, points, styles);
                }
                GraphicsPrimitive::RuPath { data, styles } => {
                    CanvasPainter::draw_path(&self.context, data, styles);
                }
            }
        }
    }

    fn draw_rect(context: &web_sys::CanvasRenderingContext2d, data: &RectangleData, styles: &Styles) {
        if data.rx > 0 || data.ry > 0 {
            CanvasPainter::draw_round_rect(context, data, styles);
        } else {
            match styles {
                Styles::Fill { color } => {
                    CanvasPainter::apply_fill(context, color);
                    context.fill_rect(
                        data.left as f64,
                        data.top as f64,
                        data.width as f64,
                        data.height as f64
                    );
                }
                Styles::Stroke { color, width } => {
                    CanvasPainter::apply_stroke(context, color, *width as f64);
                    context.stroke_rect(
                        data.left as f64,
                        data.top as f64,
                        data.width as f64,
                        data.height as f64
                    );
                }
            }
        }
    }

    fn draw_round_rect(context: &web_sys::CanvasRenderingContext2d, data: &RectangleData, styles: &Styles) {
        context.begin_path();
        context.move_to(
            (data.left + data.rx) as f64,
            data.top as f64
        );
        context.line_to(
            (data.left + data.width - data.rx) as f64,
            data.top as f64
        );
        context.quadratic_curve_to(
            (data.left + data.width) as f64,
            data.top as f64,
            (data.left + data.width) as f64,
            (data.top + data.ry) as f64
        );
        context.line_to(
            (data.left + data.width) as f64,
            (data.top + data.height - data.ry) as f64
        );
        context.quadratic_curve_to(
            (data.left + data.width) as f64,
            (data.top + data.height) as f64,
            (data.left + data.width - data.rx) as f64,
            (data.top + data.height) as f64
        );
        context.line_to(
            (data.left + data.rx) as f64,
            (data.top + data.height) as f64
        );
        context.quadratic_curve_to(
            data.left as f64,
            (data.top + data.height) as f64,
            data.left as f64,
            (data.top + data.height - data.ry) as f64
        );
        context.line_to(
            data.left as f64,
            (data.top + data.ry) as f64
        );
        context.quadratic_curve_to(
            data.left as f64,
            data.top as f64,
            (data.top + data.rx) as f64,
            data.top as f64
        );

        CanvasPainter::fill_or_stroke(context, styles);

        context.close_path();
    }

    fn draw_line(context: &web_sys::CanvasRenderingContext2d, data: &LineData, styles: &Styles) {
        context.begin_path();
        context.move_to(data.x1 as f64, data.y1 as f64);
        context.line_to(data.x2 as f64, data.y2 as f64);
        CanvasPainter::fill_or_stroke(context, styles);
        context.close_path();
    }

    fn draw_ellipse(context: &web_sys::CanvasRenderingContext2d, data: &EllipseData, styles: &Styles) {
        context.begin_path();
        context.ellipse(
            data.cx as f64,
            data.cy as f64,
            data.rx as f64,
            data.ry as f64,
            0.0,
            0.0,
            2.0 * f64::consts::PI
        );
        CanvasPainter::fill_or_stroke(context, styles);
        context.close_path();
    }

    fn draw_text(context: &web_sys::CanvasRenderingContext2d, data: &TextData, styles: &Styles) {
        context.set_font(&data.font[..]);

        match styles {
            Styles::Fill { color } => {
                CanvasPainter::apply_fill(context, color);
                context.fill_text(&data.text[..], data.x as f64, data.y as f64);
            }
            Styles::Stroke { color, width } => {
                CanvasPainter::apply_stroke(context, color, *width as f64);
                context.stroke_text(&data.text[..], data.x as f64, data.y as f64);
            }
        }
    }

    fn draw_polygon(context: &web_sys::CanvasRenderingContext2d, points: &Vec<Point>, styles: &Styles) {
        context.begin_path();
        context.move_to(points[0].x as f64, points[0].y as f64);
        
        for p in points.iter() {
            context.line_to(p.x as f64, p.y as f64);
        }
        context.line_to(points[0].x as f64, points[0].y as f64);

        CanvasPainter::fill_or_stroke(context, styles);

        context.close_path();
    }

    fn draw_path(context: &web_sys::CanvasRenderingContext2d, data: &PathData, styles: &Styles) {
        context.begin_path();
        context.move_to(data.start_x as f64, data.start_y as f64);

        for segment in data.segments.iter() {
            CanvasPainter::draw_path_segment(context, segment);
        }

        CanvasPainter::fill_or_stroke(context, styles);
        context.close_path();
    }
    
    fn fill_or_stroke(context: &web_sys::CanvasRenderingContext2d, styles: &Styles) {
        match styles {
            Styles::Fill { color } => {
                CanvasPainter::apply_fill(context, color);
                context.fill();
            }
            Styles::Stroke { color, width } => {
                CanvasPainter::apply_stroke(context, color, *width as f64);
                context.stroke();
            }
        }
    }

    fn apply_fill(context: &web_sys::CanvasRenderingContext2d, color: &String) {
        let js_color = JsValue::from_str(&color[..]);
        context.set_fill_style(&js_color);
    }

    fn apply_stroke(context: &web_sys::CanvasRenderingContext2d, color: &String, width: f64) {
        let js_color = JsValue::from_str(&color[..]);
        context.set_stroke_style(&js_color);
        context.set_line_width(width);
    }

    fn draw_path_segment(context: &web_sys::CanvasRenderingContext2d, segment: &PathSegment) {
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


#[derive(PartialEq, Debug)]
pub enum GeometricPrimitive {
    Rectangle { data: RectangleData },
    Line { data: LineData },
    Ellipse { data: EllipseData },
    Text { data: TextData },
    Polygon { points: Vec<Point> },
    RuPath { data: PathData }
}

pub struct StyledGeometricPrimitive {
    pub primitive: GeometricPrimitive,
    pub styles: Styles
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct RectangleData {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
    pub rx: f64,
    pub ry: f64,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct LineData {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct EllipseData {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
}

#[derive(PartialEq, Debug, Clone)]
pub struct TextData {
    pub x: f64,
    pub y: f64,
    pub text: String,
    pub font: String,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PathSegment {
    MoveTo { to_x: f64, to_y: f64 },
    LineTo { to_x: f64, to_y: f64 },
    BezierCurveTo { cp1_x: f64, cp1_y: f64, cp2_x: f64, cp2_y: f64, to_x: f64, to_y: f64 },
    QuadraticCurveTo { cp_x: f64, cp_y: f64, to_x: f64, to_y: f64 }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PathData {
    pub start_x: f64,
    pub start_y: f64,
    pub segments: Vec<PathSegment>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Styles {
    Fill { color: String },
    Stroke { color: String, width: f64 }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Transform {
    Translate { x: f64, y: f64 },
    Rotate { a: f64, x: f64, y: f64 },
    Scale { x: f64, y: f64 },
    Skew { x: f64, y: f64 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_rectangle() {
        GeometricPrimitive::Rectangle {
            data: RectangleData {
                left: 100.0,
                top: 50.0,
                width: 200.0,
                height: 75.0,
                rx: 0.0,
                ry: 5.0
            }
        };
    }

    #[test]
    fn make_ellipse() {
        GeometricPrimitive::Ellipse {
            data: EllipseData {
                cx: 60.0,
                cy: 70.0,
                rx: 30.0,
                ry: 45.0
            }
        };
    }

    #[test]
    fn make_line() {
        GeometricPrimitive::Line {
            data: LineData {
                x1: 1.0,
                y1: 2.0,
                x2: 11.0,
                y2: 12.0
            }
        };
    }

    #[test]
    fn make_text() {
        GeometricPrimitive::Text {
            data: TextData {
                x: 900.0,
                y: 1000.0,
                text: "Hello".to_string(),
                font: "Dialog".to_string()
            }
        };
    }

    #[test]
    fn make_polygon() {
        GeometricPrimitive::Polygon {
            points: vec![
                Point { x: 4.0, y: 41.0 },
                Point { x: 5.0, y: 45.0 }
            ]
        };
    }

    #[test]
    fn make_path() {
        GeometricPrimitive::RuPath {
            data: PathData {
                start_x: 10.0,
                start_y: 20.0,
                segments: vec![
                    PathSegment::MoveTo { to_x: 30.0, to_y: 40.0 },
                    PathSegment::LineTo { to_x: 50.0, to_y: 20.0 },
                    PathSegment::BezierCurveTo {
                        cp1_x: 100.0,
                        cp1_y: 300.0,
                        cp2_x: 400.0,
                        cp2_y: 500.0,
                        to_x: 225.0,
                        to_y: 275.0
                    },
                    PathSegment::QuadraticCurveTo {
                        cp_x: 2100.0,
                        cp_y: 1800.0,
                        to_x: 1922.0,
                        to_y: 1918.0
                    }
                ]
            }
        };
    }
}

#[derive(PartialEq, Debug)]
pub enum GraphicsPrimitive {
    Rectangle { data: RectangleData },
    Line { data: LineData },
    Ellipse { data: EllipseData },
    Text { data: TextData },
    Polyline { points: Vec<Point> },
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
pub struct TextData {
    pub x: u32,
    pub y: u32,
    pub text: String,
    pub font: String,
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
            text: "Hello".to_string(),
            font: "Dialog".to_string()
        }};
    }

    #[test]
    fn make_polyline() {
        GraphicsPrimitive::Polyline { points: vec![
            Point { x: 1, y: 11 },
            Point { x: 2, y: 21 },
            Point { x: 3, y: 31 }
        ]};
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

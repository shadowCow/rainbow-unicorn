use std::f64;

pub mod primitives;
use primitives::*;

pub mod timelines;
use timelines::*;

pub trait Painter {
    fn clear(&self);
    fn draw_rect(&self, data: &RectangleData, styles: &Styles);
    fn draw_line(&self, data: &LineData, styles: &Styles);
    fn draw_ellipse(&self, data: &EllipseData, styles: &Styles);
    fn draw_text(&self, data: &TextData, styles: &Styles);
    fn draw_polygon(&self, points: &Vec<Point>, styles: &Styles);
    fn draw_path(&self, data: &PathData, styles: &Styles);
}

fn paint<T: Painter>(painter: &T, graphics_primitives: &Vec<GraphicsPrimitive>) {
    painter.clear();
    for gp in graphics_primitives.iter() {
        match gp {
            GraphicsPrimitive::Rectangle { data, styles } => {
                painter.draw_rect(data, styles);
            }
            GraphicsPrimitive::Line { data, styles } => {
                painter.draw_line(data, styles);
            }
            GraphicsPrimitive::Ellipse { data, styles } => {
                painter.draw_ellipse(data, styles);
            }
            GraphicsPrimitive::Text { data, styles } => {
                painter.draw_text(data, styles);
            }
            GraphicsPrimitive::Polygon { points, styles } => {
                painter.draw_polygon(points, styles);
            }
            GraphicsPrimitive::RuPath { data, styles } => {
                painter.draw_path(data, styles);
            }
        }
    }
}

pub trait StateContainer {
    fn update(&self, timestamp_millis: f64) -> Vec<GraphicsPrimitive>;
}

pub fn tick<T: StateContainer, P: Painter>(
    timestamp_millis: f64,
    state_container: &T,
    painter: &P
) {
    let new_primitives = state_container.update(timestamp_millis);
    paint(painter, &new_primitives);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockPainter {}
    impl Painter for MockPainter {
        fn clear(&self) {}
        fn draw_rect(&self, data: &RectangleData, styles: &Styles) {}
        fn draw_line(&self, data: &LineData, styles: &Styles) {}
        fn draw_ellipse(&self, data: &EllipseData, styles: &Styles) {}
        fn draw_text(&self, data: &TextData, styles: &Styles) {}
        fn draw_polygon(&self, points: &Vec<Point>, styles: &Styles){}
        fn draw_path(&self, data: &PathData, styles: &Styles) {}
    }

    struct MockStateContainer {}
    impl StateContainer for MockStateContainer {
        fn update(&self, timestamp_millis: f64) -> Vec<GraphicsPrimitive> {
            vec![]
        }
    }

    #[test]
    fn test_rainbow_unicorn() {
        // need a mock painter and mock state container that we can do validations on.

        // for now, just do something that will at least run the code.
        let mock_painter = MockPainter {};
        let mock_state_container = MockStateContainer {};

        let timestamp_millis = 0.0;

        tick(timestamp_millis, &mock_state_container, &mock_painter);
    }
}
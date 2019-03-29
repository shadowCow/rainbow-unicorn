
#[derive(PartialEq, Debug)]
pub enum GraphicsPrimitive {
    Rectangle { data: RectangleData },
    Line { data: LineData },
}

#[derive(PartialEq, Debug)]
pub struct RectangleData {
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(PartialEq, Debug)]
pub struct LineData {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub trait FlexContainer {}
pub trait FlexItem {}
pub trait Canvas {
    fn dimensions(&self) -> Dimensions;
}

pub struct Dimensions {
    width: usize,
    height: usize,
}

pub struct Flexbox<T: FlexContainer> {
    element: T,
}

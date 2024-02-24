use pixels::wgpu::Color;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ElementKind {
    Sand,
    Empty,
}


#[derive(Copy, Clone, Debug)]
pub struct Element {
    pub(super) kind: ElementKind,
    pub(super) colour: Color,
}

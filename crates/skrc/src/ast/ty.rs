#[derive(Debug, PartialEq, Clone)]
pub struct PathSegment {
    pub ident: super::core::Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Path(Path),
    // TODO: Array, Slice, ...
}

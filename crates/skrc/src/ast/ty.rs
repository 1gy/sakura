#[derive(Debug, PartialEq, Clone)]
pub struct PathSegment {
    pub ident: super::core::Identifier,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    Path(Path),
    // TODO: Array, Slice, ...
}

#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    pub id: super::core::NodeId,
    pub kind: TypeKind,
}

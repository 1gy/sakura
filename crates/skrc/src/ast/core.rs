#[skrc_branded::branded]
pub type NodeId = u32;

#[skrc_branded::branded]
pub type Symbol = String;

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub symbol: Symbol,
}

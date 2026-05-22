use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegimeType {
    Democracy,
    Authoritarian,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nation {
    pub name: String,
    pub regime: RegimeType,
    pub public_approval: u8,
    pub energy: u32,
    pub industry: u32,
    pub technology: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorldState {
    pub turn: u32,
    pub nations: BTreeMap<String, Nation>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            turn: 0,
            nations: BTreeMap::new(),
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new()
    }
}

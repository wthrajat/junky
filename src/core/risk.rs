use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Risk {
    Safe,
    Aggressive,
}

impl Risk {
    pub fn label(self) -> &'static str {
        match self {
            Risk::Safe => "safe",
            Risk::Aggressive => "aggressive",
        }
    }

    pub fn is_aggressive(self) -> bool {
        matches!(self, Risk::Aggressive)
    }
}

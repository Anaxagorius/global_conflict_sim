use crate::core::model::WorldState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnPhase {
    Governance,
    Diplomacy,
    Economy,
    Conflict,
    Information,
}

pub fn phase_order() -> [TurnPhase; 5] {
    [
        TurnPhase::Governance,
        TurnPhase::Diplomacy,
        TurnPhase::Economy,
        TurnPhase::Conflict,
        TurnPhase::Information,
    ]
}

pub fn resolve_turn(state: &mut WorldState) {
    for _phase in phase_order() {
        // Placeholder for deterministic phase logic.
    }
    state.turn += 1;
}

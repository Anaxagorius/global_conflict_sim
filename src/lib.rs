pub mod core;

#[cfg(test)]
mod tests {
    use crate::core::model::WorldState;
    use crate::core::turn::resolve_turn;

    #[test]
    fn turn_resolution_advances_turn_counter() {
        let mut state = WorldState::new();
        resolve_turn(&mut state);
        assert_eq!(state.turn, 1);
    }
}

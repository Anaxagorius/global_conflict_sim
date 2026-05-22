use global_conflict_sim::core::model::WorldState;
use global_conflict_sim::core::turn::resolve_turn;

fn main() {
    let mut world = WorldState::new();
    resolve_turn(&mut world);
    println!(
        "Kaleb's Modern World bootstrap initialized at turn {}",
        world.turn
    );
}

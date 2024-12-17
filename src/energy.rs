use bevy::prelude::*;

#[derive(Component)]
pub struct Energy(pub f32);

pub fn energy_system(time: Res<Time>, mut query: Query<(Entity, &mut Energy)>, par_commands: ParallelCommands) {
    query.par_iter_mut().for_each(|(entity, mut energy)| {
        energy.0 -= 1.0 * time.delta_secs();
        if energy.0 <= 0.0 {
            par_commands.command_scope(|mut commands| {
                commands.entity(entity).despawn_recursive();
            });
        }
    });
}
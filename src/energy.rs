use bevy::prelude::*;

#[derive(Component)]
pub struct Energy(pub f32);

pub fn energy_system(time: Res<Time>, mut query: Query<(Entity, &mut Energy)>, mut commands: Commands) {
    query.iter_mut().for_each(|(entity, mut energy)| {
        energy.0 -= 1.0 * time.delta_secs();
        if energy.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    });
}
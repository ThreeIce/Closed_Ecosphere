use bevy::prelude::*;

#[derive(Component)]
pub struct Energy(pub f32);

pub fn energy_system(time: Res<Time>, mut query: Query<(&mut Energy)>) {
    query.par_iter_mut().for_each(|(mut energy)| {
        energy.0 -= 1.0 * time.delta_secs();
    });
}
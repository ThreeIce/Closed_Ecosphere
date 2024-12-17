use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Age(Timer);

impl Age{
    pub fn from_age(age: f32) -> Self {
        Age(Timer::from_seconds(age, TimerMode::Once))
    }
}

pub fn aging_system(time: Res<Time>,
                    mut query: Query<(Entity, &mut Age)>,
                    par_commands: ParallelCommands){
    query.par_iter_mut().for_each(|(entity, mut age)| {
        if age.tick(time.delta()).just_finished() {
            par_commands.command_scope(|mut commands| {
                commands.entity(entity).despawn_recursive();
            });
        }
    });
}
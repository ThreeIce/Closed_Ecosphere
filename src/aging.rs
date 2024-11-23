use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
struct Age(Timer);

impl Age{
    fn from_age(age: f32) -> Self {
        Age(Timer::from_seconds(age, TimerMode::Once))
    }
}

pub fn aging_system(time: Res<Time>,
                    mut query: Query<(Entity, &mut Age)>,
                    mut commands: Commands){
    for (entity, mut age) in &mut query {
        if age.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
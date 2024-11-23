use bevy::prelude::*;

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub direction: Vec2,
}

pub fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Movement)>) {
    query.par_iter_mut().for_each(|(mut transform, movement)| {
        transform.translation += Vec3::new(movement.direction.x, movement.direction.y, 0.0)
            * movement.speed
            * time.delta_secs();
    });
}

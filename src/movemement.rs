use bevy::prelude::*;
use crate::spatial_index::*;
use crate::type_component::TypeComponent;

#[derive(Component)]
pub struct Movement {
    pub speed: f32,
    pub direction: Vec2,
}

#[derive(Component, Deref, DerefMut)]
pub struct MyPosition(pub Vec2);

#[derive(Bundle)]
pub struct MovementBundle {
    pub movement: Movement,
    pub position: MyPosition,
}

// 应该被放在 post fixedupdate 里
pub fn movement_update(time: Res<Time>, mut query: Query<(&mut MyPosition, &Movement)>) {
    query.par_iter_mut().for_each(|(mut pos, movement)| {
        if movement.direction != Vec2::ZERO {
            pos.0 += movement.direction * movement.speed * time.delta_secs();
        }
    });
}
// 应该被放在 post fixedupdate 里并严格置于 movement_update 之后
pub fn index_update<T: Component + TypeComponent>(query: Query<(Entity, &MyPosition), Changed<MyPosition>>, mut index: ResMut<SpatialIndex<T>>) {
    query.iter().for_each(|(entity, pos)| {
        index.update(entity, pos.0);
    });
}
pub fn movement_sync(time: Res<Time<Fixed>>, mut query: Query<(&mut Transform, &Movement, &MyPosition)>) {
    query.par_iter_mut().for_each(|(mut xf, movement, pos)| {
        if xf.translation.x != pos.0.x || xf.translation.y != pos.0.y {
            let a = time.overstep_fraction();
            let future_position = pos.0 + movement.speed * movement.direction * time.delta_secs();
            let xy = pos.0.lerp(future_position, a);
            xf.translation.x = xy.x;
            xf.translation.y = xy.y;
        }
    });
}

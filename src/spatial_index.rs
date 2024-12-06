use bevy::log::error;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

const CELL_SIZE: f32 = 64.0;

#[derive(Resource)]
pub struct SpatialIndex<T> {
    map: HashMap<(i32, i32), HashSet<Entity>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> SpatialIndex<T> {
    pub fn get_nearby(&self, pos: Vec2, max_tile: i32) -> Vec<Entity> {
        let tile = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );
        let mut nearby = Vec::new();
        for x in -max_tile..max_tile +1 {
            for y in -max_tile..max_tile +1 {
                if let Some(mines) = self.map.get(&(tile.0 + x, tile.1 + y)) {
                    nearby.extend(mines.iter());
                }
            }
        }
        nearby
    }
    pub fn remove(&mut self, pos: Vec2, entity: Entity) {
        let tile = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );
        if let Some(mines) = self.map.get_mut(&tile) {
            mines.remove(&entity);
        } else{
            error!("No mines at {:?}", tile);
        }
    }
    pub fn insert(&mut self, pos: Vec2, entity: Entity) {
        let tile = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );
        self.map.entry(tile).or_default().insert(entity);
    }
    pub fn is_moved(&self, old_pos: Vec2, new_pos: Vec2) -> bool {
        (old_pos.x / CELL_SIZE).floor() as i32 != (new_pos.x / CELL_SIZE).floor() as i32
            || (old_pos.y / CELL_SIZE).floor() as i32 != (new_pos.y / CELL_SIZE).floor() as i32
    }
    pub fn get_in_radius(&self, pos: Vec2, radius: f32, query: Query<&Transform>) -> Vec<Entity> {
        // 向上取整
        let mut nearby = self.get_nearby(pos, (radius / CELL_SIZE).ceil() as i32);
        nearby.retain(|e| {
            if let Ok(transform) = query.get(*e) {
                transform.translation.xy().distance(pos) < radius
            } else {
                error!("Error in get_in_radius, query.get(*e) failed");
                false
            }
        });
        nearby
    }
}
impl<T> Default for SpatialIndex<T>{
    fn default() -> Self {
        SpatialIndex {
            map: HashMap::default(),
            _marker: std::marker::PhantomData,
        }
    }
}
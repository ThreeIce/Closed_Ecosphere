use bevy::log::error;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};
use crate::movemement::MyPosition;
use crate::type_component::TypeComponent;

const CELL_SIZE: f32 = 64.0;

#[derive(Resource)]
pub struct SpatialIndex<T: Component + TypeComponent> {
    tile_map: HashMap<(i32, i32), HashSet<Entity>>,
    entity_map: HashMap<Entity, Vec2>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component + TypeComponent> SpatialIndex<T> {
    pub fn get_nearby(&self, pos: Vec2, max_tile: i32) -> Vec<Entity> {
        let tile = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );
        let mut nearby = Vec::new();
        for x in -max_tile..max_tile +1 {
            for y in -max_tile..max_tile +1 {
                if let Some(mines) = self.tile_map.get(&(tile.0 + x, tile.1 + y)) {
                    nearby.extend(mines.iter());
                }
            }
        }
        nearby
    }
    pub fn remove(&mut self, entity: Entity) {
        if let Some(pos) = self.entity_map.remove(&entity) {
            let tile = (
                (pos.x / CELL_SIZE).floor() as i32,
                (pos.y / CELL_SIZE).floor() as i32,
            );
            if let Some(mines) = self.tile_map.get_mut(&tile) {
                mines.remove(&entity);
                if mines.is_empty(){
                    self.tile_map.remove(&tile);
                }
            } else {
                error!("No mines at {:?}", tile);
            }
        }else{
            error!("No position for entity {:?}", entity);
        }
    }
    pub fn insert(&mut self, entity: Entity, pos: Vec2) {
        let tile = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );
        self.tile_map.entry(tile).or_default().insert(entity);
        self.entity_map.insert(entity, pos);
    }
    pub fn is_moved(&self, old_pos: Vec2, new_pos: Vec2) -> bool {
        (old_pos.x / CELL_SIZE).floor() as i32 != (new_pos.x / CELL_SIZE).floor() as i32
            || (old_pos.y / CELL_SIZE).floor() as i32 != (new_pos.y / CELL_SIZE).floor() as i32
    }
    pub fn get_in_radius(&self, pos: Vec2, radius: f32) -> Vec<Entity> {
        // 向上取整
        let mut nearby = self.get_nearby(pos, (radius / CELL_SIZE).ceil() as i32);
        nearby.retain(|e| {
            if let Some(entity_pos) = self.entity_map.get(e) {
                entity_pos.distance(pos) < radius
            } else {
                error!("Error in get_in_radius, query.get(*e) failed");
                false
            }
        });
        nearby
    }
    pub fn get_nearest(&self, pos: Vec2) -> Option<Entity> {
        if self.tile_map.is_empty() {
            return None;
        }
        let nearby = self.get_nearby(pos, 2);
        let mut min_distance = f32::MAX;
        let mut nearest = None;
        if !nearby.is_empty() {
            for e in nearby {
                if let Some(entity_pos) = self.entity_map.get(&e) {
                    let distance = entity_pos.distance(pos);
                    if distance < min_distance {
                        min_distance = distance;
                        nearest = Some(e);
                    }
                } else {
                    error!("Error in get_nearest, query.get(*e) failed");
                }
            }
            nearest
        } else {
            for e in self.tile_map.values().flatten() {
                if let Some(entity_pos) = self.entity_map.get(e) {
                    let distance = entity_pos.distance(pos);
                    if distance < min_distance {
                        min_distance = distance;
                        nearest = Some(*e);
                    }
                } else {
                    error!("Error in get_nearest, query.get(*e) failed");
                }
            }
            nearest
        }
    }
    pub fn update(&mut self, entity: Entity, pos: Vec2) {
        if let Some(old_pos) = self.entity_map.get(&entity) {
            if self.is_moved(*old_pos, pos) {
                self.remove(entity);
                self.insert(entity, pos);
            }else{
                self.entity_map.insert(entity, pos);
            }
        }
    }
    pub fn get_pos(&self, entity: Entity) -> Option<Vec2> {
        self.entity_map.get(&entity).copied()
    }
}
impl<T: Component + TypeComponent> Default for SpatialIndex<T>{
    fn default() -> Self {
        SpatialIndex {
            tile_map: HashMap::default(),
            entity_map: HashMap::default(),
            _marker: std::marker::PhantomData,
        }
    }
}
pub fn on_entity_birth<T: TypeComponent>(
    trigger: Trigger<OnAdd, T>,
    query: Query<(Entity, &MyPosition),With<T>>,
    mut index: ResMut<SpatialIndex<T>>
){
    let pos = query.get(trigger.entity()).unwrap().1.0;
    index.insert(trigger.entity(), pos);
}

pub fn on_entity_death<T: TypeComponent>(
    trigger: Trigger<OnRemove, T>,
    mut index: ResMut<SpatialIndex<T>>
){
    index.remove(trigger.entity());
}

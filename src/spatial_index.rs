use std::collections::HashMap;
use bevy::math::Vec2;
use bevy::prelude::*;
use crate::movemement::MyPosition;
use crate::type_component::TypeComponent;
use kdtree::KdTree;
use num_traits::Float;

pub fn euclidean<T: Float>(a: &[T], b: &[T]) -> T {
    debug_assert_eq!(a.len(), b.len());
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| ((*x) - (*y)) * ((*x) - (*y)))
        .fold(T::zero(), ::std::ops::Add::add)
        .sqrt()
}

#[derive(Resource)]
pub struct SpatialIndex<T: Component + TypeComponent> {
    kd_tree: KdTree<f32, Entity, [f32;2]>,
    entity_map: HashMap<Entity, Vec2>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Component + TypeComponent> SpatialIndex<T> {
    pub fn remove(&mut self, entity: Entity) {
        self.kd_tree.remove(&self.entity_map[&entity].into(), &entity).unwrap();
        self.entity_map.remove(&entity).unwrap();
    }
    pub fn insert(&mut self, entity: Entity, pos: Vec2) {
        self.kd_tree.add([pos.x, pos.y], entity).unwrap();
        self.entity_map.insert(entity, pos);
    }
    pub fn get_in_radius(&self, pos: Vec2, radius: f32) -> Vec<(f32,&Entity)> {
        self.kd_tree.within(&[pos.x, pos.y], radius, &euclidean).unwrap_or_else(|e| {
            panic!("Error in get_in_radius: {:?}", e);
        })
    }
    pub fn get_nearest(&self, pos: Vec2) -> Option<(f32,&Entity)> {
        let nearest = self.kd_tree.nearest(&[pos.x, pos.y], 1, &euclidean).unwrap_or_else(|e| {
            panic!("Error in get_nearest: {:?}", e);
        });
        if nearest.len() > 0 {
            Some(nearest[0])
        } else {
            None
        }
    }
    ///
    /// 若 index 内包含了实体自身，通过这个方法获得第二近的实体
    ///
    pub fn get_second_nearest(&self, pos: Vec2) -> Option<(f32,&Entity)> {
        let nearest = self.kd_tree.nearest(&[pos.x, pos.y], 2, &euclidean).unwrap_or_else(|e| {
            panic!("Error in get_second_nearest: {:?}", e);
        });
        if nearest.len() > 1 {
            Some(nearest[1])
        } else {
            None
        }
    }
    pub fn update(&mut self, entity: Entity, pos: Vec2) {
        let old_pos = self.entity_map.get(&entity).unwrap();
        self.kd_tree.remove(&[old_pos.x,old_pos.y], &entity).unwrap();
        self.kd_tree.add([pos.x, pos.y], entity).unwrap();
        self.entity_map.insert(entity, pos);
    }
    pub fn get_pos(&self, entity: Entity) -> Option<Vec2> {
        self.entity_map.get(&entity).copied()
    }
}
impl<T: Component + TypeComponent> Default for SpatialIndex<T>{
    fn default() -> Self {
        SpatialIndex {
            kd_tree: KdTree::new(2),
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

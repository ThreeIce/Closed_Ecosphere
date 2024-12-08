use bevy::prelude::*;
use crate::health::*;
use crate::grass_reproduction::*;
use crate::aging::*;
use crate::config::*;
use crate::movemement::MyPosition;
use crate::type_component::TypeComponent;

#[derive(Component)]
pub struct Grass;

impl TypeComponent for Grass {}

#[derive(Bundle)]
pub struct GrassBundle {
    pub grass: Grass,
    pub health: Health,
    pub timer: GrassReproductionTimer,
    pub neighbor_count: GrassNeighborCount,
    pub age: Age,
    // 渲染相关
    pub mesh2d: Mesh2d,
    pub mesh_material2d: MeshMaterial2d<ColorMaterial>,
    // 位置
    pub transform: Transform,
    pub my_pos: MyPosition,
}
impl GrassBundle {
    pub fn from_config(config: &Res<Config>, x: f32, y: f32) -> Self {
        GrassBundle {
            grass: Grass,
            health: Health(config.grass_health.clone()),
            timer: GrassReproductionTimer::from_reproduction_delta(config.grass_reproduction_delta),
            neighbor_count: GrassNeighborCount(0),
            age: Age::from_age(config.grass_age),
            mesh2d: Mesh2d(config.grass_shape.clone()),
            mesh_material2d: MeshMaterial2d(config.grass_material.clone()),
            transform: Transform::from_xyz(x, y, 0.0),
            my_pos: MyPosition(Vec2::new(x, y)),
        }
    }
}
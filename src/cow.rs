use bevy::prelude::*;
use crate::aging::Age;
use crate::health::Health;
use crate::type_component::TypeComponent;

#[derive(Component)]
pub struct Cow;
impl TypeComponent for Cow {}

#[derive(Bundle)]
pub struct CowBundle {
    pub cow: Cow,
    pub health: Health,
    pub age: Age,
    // 渲染相关
    pub mesh2d: Mesh2d,
    pub mesh_material2d: MeshMaterial2d<ColorMaterial>,
    // 位置
    pub transform: Transform,
}
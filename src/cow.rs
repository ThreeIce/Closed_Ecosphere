use bevy::prelude::*;
use crate::aging::Age;
use crate::config::Config;
use crate::cow_agent::{CowAgent, CowState};
use crate::energy::Energy;
use crate::from_config::FromConfig;
use crate::health::Health;
use crate::movemement::{Movement, MyPosition};

#[derive(Bundle)]
pub struct CowBundle {
    pub health: Health,
    pub age: Age,
    // Agent
    pub cow_agent: CowAgent,
    pub energy: Energy,
    // 渲染相关
    pub mesh2d: Mesh2d,
    pub mesh_material2d: MeshMaterial2d<ColorMaterial>,
    // 位置
    pub transform: Transform,
    pub my_pos: MyPosition,
    pub movement: Movement,
}
impl FromConfig for CowBundle {
    fn from_config(config: &Res<Config>, x: f32, y: f32) -> Self {
        CowBundle {
            health: Health(config.cow_health),
            age: Age::from_age(config.cow_age),
            cow_agent: CowAgent{
                state: CowState::Idle,
                timer: Timer::from_seconds(0.0, TimerMode::Once),
                target: None,
                last_energy_gain: 0.0,
            },
            energy: Energy(config.cow_energy),
            mesh2d: Mesh2d(config.cow_shape.clone()),
            mesh_material2d: MeshMaterial2d(config.cow_material.clone()),
            transform: Transform::from_xyz(x, y, 0.0),
            my_pos: MyPosition(Vec2::new(x, y)),
            movement: Movement{
                speed: config.cow_speed,
                direction: Vec2::new(0.0, 0.0),
            },
        }
    }
}
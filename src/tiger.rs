use bevy::prelude::*;
use crate::aging::Age;
use crate::config::Config;
use crate::energy::Energy;
use crate::from_config::FromConfig;
use crate::health::Health;
use crate::movemement::{Movement, MyPosition};
use crate::tiger_agent::{TigerAgent, TigerState};

#[derive(Bundle)]
pub struct TigerBundle {
    pub health: Health,
    pub age: Age,
    // Agent
    pub tiger_agent: TigerAgent,
    pub energy: Energy,
    // 渲染相关
    pub mesh2d: Mesh2d,
    pub mesh_material2d: MeshMaterial2d<ColorMaterial>,
    // 位置
    pub transform: Transform,
    pub my_pos: MyPosition,
    pub movement: Movement,
}
impl FromConfig for TigerBundle {
    fn from_config(config: &Res<Config>, x: f32, y: f32) -> Self {
        TigerBundle {
            health: Health(config.tiger_health),
            age: Age::from_age(config.tiger_age),
            tiger_agent: TigerAgent{
                state: TigerState::Idle,
                timer: Timer::from_seconds(0.0, TimerMode::Once),
                target: None,
                last_energy_gain: 0.0,
            },
            energy: Energy(config.tiger_energy),
            mesh2d: Mesh2d(config.tiger_shape.clone()),
            mesh_material2d: MeshMaterial2d(config.idle_color.clone()),
            transform: Transform::from_xyz(x, y, 2.0),
            my_pos: MyPosition(Vec2::new(x, y)),
            movement: Movement{
                speed: config.tiger_speed,
                direction: Vec2::ZERO,
            },
        }
    }
}
//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support

mod grass_reproduction;
mod aging;
mod health;
mod movemement;
mod energy;
mod config;
mod spatial_index;
mod grass;
mod cow_agent;
mod cow;
mod prey_agent;
mod type_component;
mod reproduction;
mod from_config;
mod camera_control;
mod state_display;

use bevy::prelude::*;
use grass_reproduction::*;
use aging::*;
use grass::*;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::text::FontSmoothing;
use config::*;
use crate::camera_control::camera_control;
use crate::cow::*;
use crate::cow_agent::*;
use crate::energy::energy_system;
use crate::from_config::FromConfig;
use crate::movemement::{index_update, movement_sync, movement_update};
use crate::prey_agent::*;
use crate::reproduction::{find_mate_when_energy_enough_and_idle, mating_conditions, reproduction_state_running, searching_mate_conditions, ReproductionConfig};
use crate::spatial_index::*;
use crate::state_display::cow_state_display;

fn main() {
    // 输入初始参数
    println!("Please input the initial count of grass:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let initial_grass_count: usize = input.trim().parse().unwrap();
    println!("Please input the initial count of cow:");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let initial_cow_count: usize = input.trim().parse().unwrap();
    println!("Please input the initial count of tiger:");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let initial_tiger_count: usize = input.trim().parse().unwrap();
    // 初始化 App
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,
                     FpsOverlayPlugin {
                         config: FpsOverlayConfig {
                             text_config: TextFont {
                                 // Here we define size of our overlay
                                 font_size: 42.0,
                                 // If we want, we can use a custom font
                                 font: default(),
                                 // We could also disable font smoothing,
                                 font_smoothing: FontSmoothing::default(),
                             },
                             // We can also change color of the overlay
                             text_color: Color::srgb(0.0, 1.0, 0.0),
                             enabled: true,
                         },
                     },));
    let config = Config::from(
        initial_grass_count,
        initial_cow_count,
        initial_tiger_count,
        &mut app.world_mut());
    // 初始化资源
    app.insert_resource(config.clone())
        .init_resource::<SpatialIndex<Grass>>()
        .init_resource::<SpatialIndex<CowAgent>>()
        // 插入捕猎相关资源
        .insert_resource(Damage::<CowAgent>::new(config.cow_damage))
        .insert_resource(EnergyGain::<Grass>::new(config.grass_gain))
        .insert_resource(EnergyGain::<CowAgent>::new(config.cow_gain))
        .insert_resource(AttackCoolingTime::<CowAgent>::new(config.cow_attack_cooling_time))
        .insert_resource(EatingTime::<CowAgent>::new(config.cow_eating_time))
        // 插入繁殖相关资源
        .insert_resource(ReproductionConfig::<CowAgent>{
            energy_threshold: config.cow_reproduction_energy_threshold,
            energy_cost: config.cow_reproduction_cost,
            search_radius: config.cow_search_radius,
            reproduction_radius: config.cow_reproduction_radius,
            mating_time: config.cow_mating_time,
            _marker: std::marker::PhantomData,
        })
        // 配置 StartUp 系统
        .add_systems(Startup, setup)
        // 配置 Update 系统
        .add_systems(FixedUpdate,
            // aging, grass reproduction, energym
            (aging_system, grass_reproduction_system, energy_system).chain())
        // Prey Agent
        .add_systems(FixedUpdate,
            (find_prey::<CowAgent,Grass>,
                attack::<CowAgent,Grass>,).after(energy_system).after(aging_system))
        .add_systems(FixedUpdate,
            (move_to_prey::<CowAgent,Grass>,
                on_attack_cooling::<CowAgent>,
                on_eating::<CowAgent>,)
                .after(attack::<CowAgent,Grass>)
                .after(find_prey::<CowAgent,Grass>),
            )
        // Reproduction Agent
        .add_systems(FixedUpdate,
                     (find_mate_when_energy_enough_and_idle::<CowAgent>,
                      searching_mate_conditions::<CowAgent>,
                      mating_conditions::<CowAgent, CowBundle>).after(energy_system).after(aging_system)) // TODO: 添加了虎的 attack agent 之后，要在这里添加虎的 attack 依赖
        .add_systems(FixedUpdate, reproduction_state_running::<CowAgent>
            .after(find_mate_when_energy_enough_and_idle::<CowAgent>)
            .after(searching_mate_conditions::<CowAgent>)
            .after(mating_conditions::<CowAgent, CowBundle>))
        .add_systems(FixedPostUpdate, (
            // movement
            (movement_update),
            (index_update::<CowAgent>).after(movement_update),
            ))
        .add_systems(Update, (
            movement_sync,
            ))
        .add_systems(Update, camera_control)
        .add_systems(Update, cow_state_display)
        // observers
        // grass reproduction
        .add_observer(on_grass_death)
        .add_observer(on_grass_birth)
        .add_observer(on_entity_birth::<CowAgent>)
        .add_observer(on_entity_death::<CowAgent>)
        .run();
}

fn setup(
    mut commands: Commands,
    config: Res<Config>,
) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
    // 在区域范围内随机生成指定数量个草
    for _ in 0..config.initial_grass_count {
        let x = rand::random::<f32>() * config.width - config.width / 2.0;
        let y = rand::random::<f32>() * config.height - config.height / 2.0;
        commands.spawn(GrassBundle::from_config(&config, x, y));
    }
    // 在区域范围内随机生成指定数量个牛
    for _ in 0..config.initial_cow_count {
        let x = rand::random::<f32>() * config.width - config.width / 2.0;
        let y = rand::random::<f32>() * config.height - config.height / 2.0;
        commands.spawn(CowBundle::from_config(&config, x, y));
    }
}

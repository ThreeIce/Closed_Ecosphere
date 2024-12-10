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

use bevy::prelude::*;
use grass_reproduction::*;
use aging::*;
use grass::*;
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::text::FontSmoothing;
use config::*;
use crate::cow::*;
use crate::cow_agent::*;
use crate::energy::energy_system;
use crate::movemement::{index_update, movement_sync, movement_update};
use crate::prey_agent::*;
use crate::spatial_index::*;

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
        // 配置 StartUp 系统
        .add_systems(Startup, setup)
        // 配置 Update 系统
        .add_systems(FixedUpdate,
            // aging, grass reproduction, energy
            // They can spawn/despawn entity so the must be run before the movement system
            (aging_system, grass_reproduction_system, energy_system).chain())
        .add_systems(FixedUpdate,
            // Cow Agent
            (find_prey::<CowAgent,Grass>,
                attack::<CowAgent,Grass>,).after(energy_system).after(aging_system))
        .add_systems(FixedUpdate,
            (move_to_prey::<CowAgent,Grass>,
                on_attack_cooling::<CowAgent>,
                on_eating::<CowAgent>,)
                .after(attack::<CowAgent,Grass>)
                .after(find_prey::<CowAgent,Grass>),
            )
        .add_systems(FixedPostUpdate, (
            // movement
            (movement_update),
            (index_update::<CowAgent>).after(movement_update),
            ))
        .add_systems(Update, (
            movement_sync,
            ))
        // observers
        // grass reproduction
        .add_observer(on_grass_die)
        .add_observer(on_grass_birth)
        .run();
}

fn setup(
    mut commands: Commands,
    config: Res<Config>,
) {
    // camera
    let camera = Camera2dBundle{
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    };
    commands.spawn(camera);
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

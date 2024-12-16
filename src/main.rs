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
mod tiger_agent;
mod tiger;
mod escape_system;

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
use crate::escape_system::{escape_from, EscapeConfig, EscapeTimer};
use crate::from_config::FromConfig;
use crate::movemement::{index_update, movement_sync, movement_update};
use crate::prey_agent::*;
use crate::reproduction::{find_mate_when_energy_enough_and_idle, mating_conditions, reproduction_state_running, searching_mate_conditions, ReproductionConfig};
use crate::spatial_index::*;
use crate::state_display::{cow_state_display, tiger_state_display};
use crate::tiger_agent::TigerAgent;
use crate::tiger::*;

fn main() {
    // 输入初始参数
    println!("Please input the width of the area:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let width: f32 = input.trim().parse().unwrap();
    println!("Please input the height of the area:");
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    let height: f32 = input.trim().parse().unwrap();
    println!("Please input the initial count of grass:");
    input.clear();
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
        width,
        height,
        initial_grass_count,
        initial_cow_count,
        initial_tiger_count,
        &mut app.world_mut());
    // 初始化资源
    app.init_resource::<SpatialIndex<Grass>>()
        .init_resource::<SpatialIndex<CowAgent>>()
        .init_resource::<SpatialIndex<TigerAgent>>()
        // 插入捕猎相关资源
        .insert_resource(Damage::<CowAgent>::new(config.cow_damage))
        .insert_resource(Damage::<TigerAgent>::new(config.tiger_damage))
        .insert_resource(EnergyGain::<Grass>::new(config.grass_gain))
        .insert_resource(EnergyGain::<CowAgent>::new(config.cow_gain))
        .insert_resource(EnergyGain::<TigerAgent>::new(config.tiger_gain))
        .insert_resource(AttackCoolingTime::<CowAgent>::new(config.cow_attack_cooling_time))
        .insert_resource(AttackCoolingTime::<TigerAgent>::new(config.tiger_attack_cooling_time))
        .insert_resource(EatingTime::<CowAgent>::new(config.cow_eating_time))
        .insert_resource(EatingTime::<TigerAgent>::new(config.tiger_eating_time))
        // 插入逃跑相关资源
        .insert_resource(EscapeConfig::<CowAgent>{
            flee_distance: config.cow_escape_radius,
            _marker: Default::default(),
        })
        // 插入繁殖相关资源
        .insert_resource(ReproductionConfig::<CowAgent>{
            energy_threshold: config.cow_reproduction_energy_threshold,
            energy_cost: config.cow_reproduction_cost,
            search_radius: config.cow_search_radius,
            reproduction_radius: config.cow_reproduction_radius,
            mating_time: config.cow_mating_time,
            _marker: std::marker::PhantomData,
        })
        .insert_resource(ReproductionConfig::<TigerAgent>{
            energy_threshold: config.tiger_reproduction_energy_threshold,
            energy_cost: config.tiger_reproduction_cost,
            search_radius: config.tiger_search_radius,
            reproduction_radius: config.tiger_reproduction_radius,
            mating_time: config.tiger_mating_time,
            _marker: std::marker::PhantomData,
        })
        .insert_resource(EscapeTimer::new(config.escape_update_delta_secs))
        .insert_resource(config)
        // 配置 StartUp 系统
        .add_systems(Startup, setup)
        // 配置 Update 系统
        .add_systems(FixedUpdate,
            // aging, grass reproduction, energym
            (aging_system, grass_reproduction_system, energy_system).chain())
        // 牛的逃跑系统
        .add_systems(FixedUpdate,escape_from::<CowAgent,TigerAgent>)
        // Prey Agent
        // 牛
        .add_systems(FixedUpdate,
            (find_prey::<CowAgent,Grass>,
                attack::<CowAgent,Grass>,)
                .after(energy_system)
                .after(aging_system)
                .after(escape_from::<CowAgent,TigerAgent>))
        .add_systems(FixedUpdate, (
            move_to_prey::<CowAgent,Grass>,
            on_attack_cooling::<CowAgent>,
            on_eating::<CowAgent>,)
            .after(attack::<CowAgent,Grass>)
            .after(find_prey::<CowAgent,Grass>))
        // 虎
        .add_systems(FixedUpdate, (
            find_prey::<TigerAgent, CowAgent>,
            attack::<TigerAgent, CowAgent>,)
            .after(energy_system)
            .after(aging_system))
        .add_systems(FixedUpdate, (
            move_to_prey::<TigerAgent, CowAgent>,
            on_attack_cooling::<TigerAgent>,
            on_eating::<TigerAgent>,)
            .after(attack::<TigerAgent, CowAgent>)
            .after(find_prey::<TigerAgent, CowAgent>))
        // Reproduction Agent
        // 牛
        .add_systems(FixedUpdate, (
            // Idle 状态下，优先找配偶，找不到配偶再寻找食物
            find_mate_when_energy_enough_and_idle::<CowAgent>
                .before(find_prey::<CowAgent, Grass>),
            searching_mate_conditions::<CowAgent>,
            mating_conditions::<CowAgent, CowBundle>)
            .after(energy_system)
            .after(aging_system)
            .after(escape_from::<CowAgent,TigerAgent>))
        .add_systems(FixedUpdate, reproduction_state_running::<CowAgent>
            .after(find_mate_when_energy_enough_and_idle::<CowAgent>)
            .after(searching_mate_conditions::<CowAgent>)
            .after(mating_conditions::<CowAgent, CowBundle>))
        // 虎
        .add_systems(FixedUpdate, (
            find_mate_when_energy_enough_and_idle::<TigerAgent>
                .before(find_prey::<TigerAgent, CowAgent>),
            searching_mate_conditions::<TigerAgent>,
            mating_conditions::<TigerAgent, TigerBundle>)
            .after(energy_system)
            .after(aging_system))
        .add_systems(FixedUpdate, reproduction_state_running::<TigerAgent>
            .after(find_mate_when_energy_enough_and_idle::<TigerAgent>)
            .after(searching_mate_conditions::<TigerAgent>)
            .after(mating_conditions::<TigerAgent, TigerBundle>))
        .add_systems(FixedPostUpdate, (
            // movement
            (movement_update),
            (index_update::<CowAgent>).after(movement_update),
            (index_update::<TigerAgent>).after(movement_update),
            ))
        .add_systems(Update, (
            movement_sync,
            ))
        .add_systems(Update, camera_control)
        .add_systems(Update, (cow_state_display, tiger_state_display))
        // observers
        // grass reproduction
        .add_observer(on_grass_death)
        .add_observer(on_grass_birth)
        .add_observer(on_entity_birth::<CowAgent>)
        .add_observer(on_entity_death::<CowAgent>)
        .add_observer(on_entity_birth::<TigerAgent>)
        .add_observer(on_entity_death::<TigerAgent>)
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
    // 在区域范围内随机生成指定数量个虎
    for _ in 0..config.initial_tiger_count {
        let x = rand::random::<f32>() * config.width - config.width / 2.0;
        let y = rand::random::<f32>() * config.height - config.height / 2.0;
        commands.spawn(TigerBundle::from_config(&config, x, y));
    }
}

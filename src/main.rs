//! Shows how to render simple primitive shapes with a single color.
//!
//! You can toggle wireframes with the space bar except on wasm. Wasm does not support

mod grass_reproduction;
mod aging;
mod basic;
mod bundle;
mod movemement;
mod energy;

use bevy::prelude::*;
use basic::*;
use grass_reproduction::*;
use aging::*;
use bundle::*;

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
    app.add_plugins(DefaultPlugins);
    let config = Config::from(
        initial_grass_count,
        initial_cow_count,
        initial_tiger_count,
        &mut app.world_mut());
    // 初始化资源
    app.insert_resource(config).
        init_resource::<SpatialIndex<Grass>>()
        // 配置 StartUp 系统
        .add_systems(Startup, setup)
        // 配置 Update 系统
        .add_systems(Update, (
            // aging
            (aging_system),
            // grass reproduction
            (grass_reproduction_system),
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
}

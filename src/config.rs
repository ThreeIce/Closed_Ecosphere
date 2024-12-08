use bevy::asset::Handle;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Config{
    // 模拟区域大小
    pub width: f32,
    pub height: f32,
    // 初始参数
    pub initial_grass_count: usize,
    pub initial_cow_count: usize,
    pub initial_tiger_count: usize,
    // For grass
    pub grass_health: f32,
    pub grass_age: f32, // in seconds
    // 草的繁殖间隔，按秒
    pub grass_reproduction_delta: f32, // in seconds
    // 草的一阶段繁殖概率
    pub grass_reproduction_rate_1: f32,
    // 草的二阶段繁殖概率
    pub grass_reproduction_rate_2: f32,
    // 草的繁殖半径
    pub grass_reproduction_radius: f32,
    // 草的捕食收获
    pub grass_gain: f32,
    // 草的模型
    pub grass_shape: Handle<Mesh>,
    // 草的材质
    pub grass_material: Handle<ColorMaterial>,
}

impl Config {
    pub fn from(initial_grass_count: usize,
                initial_cow_count: usize,
                initial_tiger_count: usize,
                world: &mut World,
    ) -> Self {
        Config {
            width: 4096.0,
            height: 4096.0,
            initial_grass_count,
            initial_cow_count,
            initial_tiger_count,
            grass_health: 1.0,
            grass_age: 3.0,
            grass_reproduction_delta: 1.0,
            grass_reproduction_rate_1: 0.5,
            grass_reproduction_rate_2: 0.1,
            grass_reproduction_radius: 50.0,
            grass_gain: 0.1,
            grass_shape: world.get_resource_mut::<Assets<Mesh>>()
                .unwrap().add(Circle::new(5.0)),
            grass_material: world.get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap().add(Color::srgb(0.0,1.0,0.0)),
            
        }
    }
}
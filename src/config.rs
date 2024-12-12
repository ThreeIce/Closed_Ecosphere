use bevy::asset::Handle;
use bevy::prelude::*;

#[derive(Resource)]
pub struct Config{
    // 模拟区域大小
    pub width: f32,
    pub height: f32,
    // 摄像机速度
    pub camera_speed: f32,
    pub camera_zoom_speed: f32,
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
    // For cow
    pub cow_health: f32,
    pub cow_age: f32, // in seconds
    // 牛初始能量
    pub cow_energy: f32,
    // 牛的捕食收获
    pub cow_gain: f32,
    // 牛的伤害
    pub cow_damage: f32,
    // 牛的攻击冷却时间
    pub cow_attack_cooling_time: f32,
    // 牛的吃草时间
    pub cow_eating_time: f32,
    // 牛的速度
    pub cow_speed: f32,
    // 牛的繁殖能量阈值
    pub cow_reproduction_energy_threshold: f32,
    // 牛的繁殖能量消耗
    pub cow_reproduction_cost: f32,
    // 牛的寻找伴侣半径
    pub cow_search_radius: f32,
    // 牛的繁殖半径
    pub cow_reproduction_radius: f32,
    // 牛的繁殖时间
    pub cow_mating_time: f32,
    // 牛的模型
    pub cow_shape: Handle<Mesh>,
    // 动物各状态下颜色
    pub idle_color: Handle<ColorMaterial>,
    pub hunting_color: Handle<ColorMaterial>,
    pub attack_cooling_color: Handle<ColorMaterial>,
    pub eating_color: Handle<ColorMaterial>,
    pub searching_mate_color: Handle<ColorMaterial>,
    pub mating_color: Handle<ColorMaterial>,
    pub escaping_color: Handle<ColorMaterial>
}

impl Config {
    pub fn from(
        width: f32,
        height: f32,
        initial_grass_count: usize,
        initial_cow_count: usize,
        initial_tiger_count: usize,
        world: &mut World,
    ) -> Self {
        Config {
            width: width,
            height: height,
            camera_speed: 128.0,
            camera_zoom_speed: 0.2,
            initial_grass_count,
            initial_cow_count,
            initial_tiger_count,
            grass_health: 10.0,
            grass_age: 30.0,
            grass_reproduction_delta: 8.0,
            grass_reproduction_rate_1: 0.7,
            grass_reproduction_rate_2: 0.2,
            grass_reproduction_radius: 50.0,
            grass_gain: 15.0,
            grass_shape: world.get_resource_mut::<Assets<Mesh>>()
                .unwrap().add(Circle::new(5.0)),
            grass_material: world.get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap().add(Color::srgb(0.0,1.0,0.0)),
            cow_health: 50.0,
            cow_age: 100.0,
            cow_gain: 50.0,
            cow_damage: 10.0,
            cow_attack_cooling_time: 1.0,
            cow_eating_time: 2.0,
            cow_energy: 50.0,
            cow_speed: 20.0,
            cow_reproduction_energy_threshold: 100.0,
            cow_reproduction_cost: 40.0,
            cow_search_radius: 500.0,
            cow_reproduction_radius: 40.0,
            cow_mating_time: 5.0,
            cow_shape: world.get_resource_mut::<Assets<Mesh>>()
                .unwrap().add(Rectangle::new(20.0, 20.0)),
            idle_color: world.get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap().add(Color::srgb(1.0,1.0,1.0)),
            hunting_color: world.get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap().add(Color::srgb(0.0,0.0,1.0)),
            attack_cooling_color: world.get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap().add(Color::srgb(0.0,0.5,1.0)),
            eating_color: world.get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap().add(Color::srgb(0.0,1.0,1.0)),
            searching_mate_color: world.get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap().add(Color::srgb(0.5,0.0,1.0)),
            mating_color: world.get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap().add(Color::srgb(1.0,0.0,1.0)),
            escaping_color: world.get_resource_mut::<Assets<ColorMaterial>>()
                .unwrap().add(Color::srgb(1.0,0.5,0.0)),
        }
    }
}

impl Clone for Config{
    fn clone(&self) -> Self {
        Config{
            width: self.width,
            height: self.height,
            camera_speed: self.camera_speed,
            camera_zoom_speed: self.camera_zoom_speed,
            initial_grass_count: self.initial_grass_count,
            initial_cow_count: self.initial_cow_count,
            initial_tiger_count: self.initial_tiger_count,
            grass_health: self.grass_health,
            grass_age: self.grass_age,
            grass_reproduction_delta: self.grass_reproduction_delta,
            grass_reproduction_rate_1: self.grass_reproduction_rate_1,
            grass_reproduction_rate_2: self.grass_reproduction_rate_2,
            grass_reproduction_radius: self.grass_reproduction_radius,
            grass_gain: self.grass_gain,
            grass_shape: self.grass_shape.clone(),
            grass_material: self.grass_material.clone(),
            cow_health: self.cow_health,
            cow_age: self.cow_age,
            cow_gain: self.cow_gain,
            cow_damage: self.cow_damage,
            cow_shape: self.cow_shape.clone(),
            idle_color: self.idle_color.clone(),
            hunting_color: self.hunting_color.clone(),
            attack_cooling_color: self.attack_cooling_color.clone(),
            eating_color: self.eating_color.clone(),
            searching_mate_color: self.searching_mate_color.clone(),
            mating_color: self.mating_color.clone(),
            escaping_color: self.escaping_color.clone(),
            cow_attack_cooling_time: self.cow_attack_cooling_time,
            cow_eating_time: self.cow_eating_time,
            cow_energy: self.cow_energy,
            cow_speed: self.cow_speed,
            cow_reproduction_energy_threshold: self.cow_reproduction_energy_threshold,
            cow_reproduction_cost: self.cow_reproduction_cost,
            cow_reproduction_radius: self.cow_reproduction_radius,
            cow_mating_time: self.cow_mating_time,
            cow_search_radius: self.cow_search_radius,
        }
    }
}
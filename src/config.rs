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

    // For Grass
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

    // For Cow
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
    // 牛的逃跑检测半径
    pub cow_escape_radius: f32,
    // 牛的模型
    pub cow_shape: Handle<Mesh>,

    // For Tiger
    pub tiger_health: f32,
    pub tiger_age: f32, // in seconds
    // 虎初始能量
    pub tiger_energy: f32,
    // 虎的捕食收获
    pub tiger_gain: f32,
    // 虎的伤害
    pub tiger_damage: f32,
    // 虎的攻击冷却时间
    pub tiger_attack_cooling_time: f32,
    // 虎的进食时间
    pub tiger_eating_time: f32,
    // 虎的速度
    pub tiger_speed: f32,
    // 虎的繁殖能量阈值
    pub tiger_reproduction_energy_threshold: f32,
    // 虎的繁殖能量消耗
    pub tiger_reproduction_cost: f32,
    // 虎的寻找伴侣半径
    pub tiger_search_radius: f32,
    // 虎的繁殖半径
    pub tiger_reproduction_radius: f32,
    // 虎的繁殖时间
    pub tiger_mating_time: f32,
    // 虎的模型
    pub tiger_shape: Handle<Mesh>,

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
            cow_escape_radius: 100.0,
            cow_shape: world.get_resource_mut::<Assets<Mesh>>()
                .unwrap().add(Rectangle::new(20.0, 20.0)),
            tiger_health: 100.0,
            tiger_age: 200.0,
            tiger_gain: 100.0,
            tiger_damage: 20.0,
            tiger_attack_cooling_time: 2.0,
            tiger_eating_time: 5.0,
            tiger_energy: 100.0,
            tiger_speed: 40.0,
            tiger_reproduction_energy_threshold: 200.0,
            tiger_reproduction_cost: 80.0,
            tiger_search_radius: 1000.0,
            tiger_reproduction_radius: 80.0,
            tiger_mating_time: 10.0,
            tiger_shape: world.get_resource_mut::<Assets<Mesh>>()
                .unwrap().add(RegularPolygon::new(20.0, 6)),
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
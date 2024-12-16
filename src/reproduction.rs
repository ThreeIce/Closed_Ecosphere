use bevy::prelude::*;
use bevy::reflect::Map;
use bevy::utils::{HashMap};
use crate::config::Config;
use crate::energy::Energy;
use crate::from_config::FromConfig;
use crate::movemement::{Movement, MyPosition};
use crate::spatial_index::{euclidean, SpatialIndex};
use crate::type_component::TypeComponent;
use kdtree;
use kdtree::KdTree;

pub enum ReproductionState{
    Idle,
    SearchingMate,
    Mating,
    // 处于其它状态，但是可以切换到寻找配偶状态
    OtherCanMate,
    // 处于其它状态，且不能切换到寻找配偶状态
    OtherCantMate,
}

pub trait ReproductionAgent: Component{
    fn get_state(&self) -> ReproductionState;
    fn switch_to_idle(&mut self);
    fn switch_to_searching_mate(&mut self, mate: Entity);
    fn switch_to_mating(&mut self, mating_time: f32);
    fn get_mate(&self) -> Option<Entity>;
    fn get_reproduction_timer(&mut self) -> &mut Timer;
}
#[derive(Resource)]
pub struct ReproductionConfig<T:ReproductionAgent + TypeComponent>{
    // 开始寻找配偶所需要的能量阈值
    pub energy_threshold: f32,
    // 繁殖所消耗的能量
    pub energy_cost: f32,
    // 寻找半径
    pub search_radius: f32,
    // 繁殖距离
    pub reproduction_radius: f32,
    // 繁殖所需时间
    pub mating_time: f32,
    pub _marker: std::marker::PhantomData<T>,
}

impl<T:ReproductionAgent + TypeComponent> ReproductionConfig<T>{
    pub fn new(energy_threshold: f32, energy_cost: f32, search_radius: f32, reproduction_radius: f32, mating_time: f32) -> Self{
        ReproductionConfig{
            energy_threshold,
            energy_cost,
            search_radius,
            reproduction_radius,
            mating_time,
            _marker: std::marker::PhantomData
        }
    }
}
pub fn find_mate_when_energy_enough_and_idle<T: ReproductionAgent + TypeComponent>(
    mut query: Query<(Entity, &mut T, &Energy, &MyPosition)>,
    reproduction_config: Res<ReproductionConfig<T>>,
){
    let mut entities_map = HashMap::<Entity, Vec2>::new();
    query.iter().for_each(|(entity, agent, energy, pos)|{
        match agent.get_state(){
            ReproductionState::Idle|ReproductionState::OtherCanMate => {
                if energy.0 >= reproduction_config.energy_threshold{
                    entities_map.insert(entity, pos.0);
                }
            }
            _ => {}
        }
    });
    let mut kdtree = KdTree::with_capacity(2, entities_map.len());
    for (entity, pos) in entities_map.iter() {
        kdtree.add([pos.x, pos.y], *entity).unwrap()
    }
    while entities_map.len() > 1{ //只有一个，没有找伴的意义
        let (&entity, &pos) = entities_map.iter().next().unwrap();
        // 先移除自身，避免在搜索最近点时获取的是自身
        kdtree.remove(&[pos.x, pos.y], &entity).unwrap();
        entities_map.remove(&entity);
        let (distance, &nearest_entity) = kdtree.nearest(&[pos.x, pos.y], 1, &euclidean).unwrap()[0];
        if distance <= reproduction_config.search_radius{
            let (_, mut agent, _, _) = query.get_mut(entity).unwrap();
            agent.switch_to_searching_mate(nearest_entity);
            let (_, mut mate_agent, _, _) = query.get_mut(nearest_entity).unwrap();
            mate_agent.switch_to_searching_mate(entity);
            let mate_pos = entities_map.get(&nearest_entity).unwrap();
            kdtree.remove(&[mate_pos.x, mate_pos.y], &nearest_entity).unwrap();
            entities_map.remove(&nearest_entity);
        }
    }
}
pub fn searching_mate_conditions<T: ReproductionAgent + TypeComponent>(
    mut query: Query<(Entity, &mut T, &MyPosition)>,
    reproduction_config: Res<ReproductionConfig<T>>,
) {
    let mut mating_entities = HashMap::<Entity,Entity>::new();
    query.iter().for_each(|(entity, agent, _)|{
        match agent.get_state(){
            ReproductionState::SearchingMate => {
                mating_entities.insert(entity, agent.get_mate().unwrap());
            }
            _ => {}
        }
    });
    mating_entities.iter().for_each(|(entity, mate)| {
        if let Some(mates_mate) = mating_entities.get(mate){
            if *mates_mate == *entity{
                let pos2 = query.get(*mate).unwrap().2.0;
                let (_, mut agent, pos) = query.get_mut(*entity).unwrap();
                match agent.get_state() {
                    ReproductionState::SearchingMate => {
                        if pos.0.distance(pos2) <= reproduction_config.reproduction_radius {
                            agent.switch_to_mating(reproduction_config.mating_time);
                            let (_, mut mate_agent, _) = query.get_mut(*mate).unwrap();
                            mate_agent.switch_to_mating(reproduction_config.mating_time);
                        }
                    }
                    _ => {}
                }
            }else{
                // 显然出错了，双方都处于寻找配偶状态但繁殖对象不是对方
                panic!("Error in searching_mate_conditions, entity and mate are not each other's mate");
            }
        }
        else
        {
            // 繁殖对象不处于寻找配偶状态或已经死亡，切换到 idle
            if let Ok((_, mut agent, _)) = query.get_mut(*entity){
                agent.switch_to_idle();
            }
        }
    });
}
pub fn mating_conditions<T: ReproductionAgent + TypeComponent, TB: Bundle + FromConfig>(
    mut query: Query<(Entity, &mut T, &mut Energy, &MyPosition)>,
    time: Res<Time>,
    mut commands: Commands,
    reproduction_config: Res<ReproductionConfig<T>>,
    app_config: Res<Config>
){
    let mut mating_entities = HashMap::<Entity,Entity>::new();
    query.iter().for_each(|(entity, agent, _, _)|{
        match agent.get_state(){
            ReproductionState::Mating => {
                mating_entities.insert(entity, agent.get_mate().unwrap());
            }
            _ => {}
        }
    });
    mating_entities.iter().for_each(|(entity, mate)| {
        if let Some(mates_mate) = mating_entities.get(mate){
            if *mates_mate == *entity{
                let (_, mut agent, mut energy, pos) = query.get_mut(*entity).unwrap();
                match agent.get_state() {
                    ReproductionState::Mating => {
                        let timer = agent.get_reproduction_timer();
                        timer.tick(time.delta());
                        if timer.just_finished() {
                            energy.0 -= reproduction_config.energy_cost;
                            agent.switch_to_idle();
                            let pos = pos.0; // 解引用，避免重复借用
                            let (_, mut mate_agent, mut mate_energy, mate_pos) = query.get_mut(*mate).unwrap();
                            mate_agent.switch_to_idle();
                            mate_energy.0 -= reproduction_config.energy_cost;
                            let new_pos = (pos + mate_pos.0) / 2.0;
                            commands.spawn(TB::from_config(&app_config, new_pos.x, new_pos.y));
                        }
                    }
                    ReproductionState::Idle => {
                        // 如果该实体的 mate 比该实体先被访问到，该实体有可能已经切换到 idle 状态，那就无需再做处理了。
                    }
                    _ => {
                        // 不可能，绝对不可能。
                        panic!("Error in mating_conditions, entity is not in mating state or idle state");
                    }
                }
            } else {
                // 显然出错了，双方都处于繁殖状态但繁殖对象不是对方
                panic!("Error in mating_conditions, entity and mate are not each other's mate");
            }
        }
        else
        {
            // 繁殖对象不处于繁殖状态或已经死亡，切换到 idle
            if let Ok((_, mut agent, _, _)) = query.get_mut(*entity){
                agent.switch_to_idle();
            }
        }
    });
}
pub fn reproduction_state_running<T: ReproductionAgent + TypeComponent>(
    mut query: Query<(Entity, &T, &mut Movement, &MyPosition)>,
    index: Res<SpatialIndex<T>>,
){
    // 状态运行
    query.par_iter_mut().for_each(|(entity, agent, mut movement, pos)| {
        match agent.get_state() {
            ReproductionState::SearchingMate => {
                // 寻找配偶状态下，不断更新配偶位置
                let mate = agent.get_mate().unwrap();
                // 配偶可能会刚好在状态机条件检查完，这段代码开始运行前被虎杀死，如果这种情况发生，不作为，由下一帧的状态机条件检查系统来处理。
                if let Some(mate_pos) = index.get_pos(mate) {
                    movement.direction = (mate_pos - pos.0).normalize();
                }
                else
                {
                    movement.direction = Vec2::ZERO;
                }
            }
            ReproductionState::Mating => {
                if movement.direction != Vec2::ZERO {
                    movement.direction = Vec2::ZERO;
                }
            }
            _ => {}
        }
    });
}
use bevy::prelude::*;
use bevy::utils::{HashMap};
use crate::config::Config;
use crate::energy::Energy;
use crate::from_config::FromConfig;
use crate::movemement::{Movement, MyPosition};
use crate::spatial_index::SpatialIndex;
use crate::type_component::TypeComponent;

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
    let mut combinations = query.iter_combinations_mut();
    while let Some([(entity, mut agent, energy, pos),
                   (mate, mut mate_agent, mate_energy, mate_pos)]) = combinations.fetch_next()
    {
        match agent.get_state() {
            ReproductionState::Idle => {
                // 切换条件 1：当能量足够时，尝试寻找配偶
                if energy.0 >= reproduction_config.energy_threshold
                    && mate_energy.0 >= reproduction_config.energy_threshold
                    && pos.0.distance(mate_pos.0) <= reproduction_config.search_radius {
                    match mate_agent.get_state() {
                        ReproductionState::Idle | ReproductionState::OtherCanMate => {
                            agent.switch_to_searching_mate(mate);
                            mate_agent.switch_to_searching_mate(entity);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
pub fn searching_mate_conditions<T: ReproductionAgent + TypeComponent>(
    mut query: Query<(Entity, &mut T, &MyPosition)>,
    reproduction_config: Res<ReproductionConfig<T>>,
) {
    // 切换条件 1：当与配偶进入繁殖距离，双双进入繁殖状态
    // 切换条件 2：当配偶不处于寻找配偶状态时，切换到空闲状态
    // 切换条件 3：当配偶死亡，切换到空闲状态
    // 对于每一个处于 SearchingMate 状态下的实体，都理应有一个 Mate，因此记录下所有具有 Mate 存活的实体，
    // 然后再遍历所有处于 SearchingMate 状态下的实体，如果它不在表中，说明它的 Mate 已经死亡。
    let mut combinations = query.iter_combinations_mut();

    let mut mated_set = std::collections::HashSet::new();
    while let Some([(entity, mut agent, pos),
                   (mate, mut mate_agent, mate_pos)]) = combinations.fetch_next()
    {
        match agent.get_state() {
            ReproductionState::SearchingMate => {
                if agent.get_mate().unwrap() == mate {
                    match mate_agent.get_state() {
                        ReproductionState::SearchingMate => {
                            // 切换条件 1：当与配偶进入繁殖距离，双双进入繁殖状态
                            if pos.0.distance(mate_pos.0) <= reproduction_config.reproduction_radius {
                                agent.switch_to_mating(reproduction_config.mating_time);
                                mate_agent.switch_to_mating(reproduction_config.mating_time);
                            }
                            mated_set.insert(entity);
                            mated_set.insert(mate);
                        }
                        _ => {
                            // 切换条件 2：当配偶不处于寻找配偶状态时，切换到空闲状态
                            agent.switch_to_idle();
                        }
                    }
                }
            }
            _ => {}
        }
    }
    query.iter_mut().for_each(|(entity, mut agent, _)| {
        match agent.get_state() {
            ReproductionState::SearchingMate => {
                // 切换条件 3：当配偶死亡，切换到 idle
                if !mated_set.contains(&entity) {
                    agent.switch_to_idle();
                }
            }
            _ => {}
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
    query.par_iter_mut().for_each(|(_, agent, mut movement, pos)| {
        match agent.get_state() {
            ReproductionState::SearchingMate => {
                // 寻找配偶状态下，不断更新配偶位置
                let mate = agent.get_mate().unwrap();
                // 这段代码假定条件检查系统已经运行过，如果 mate 死亡或切换状态，则遍历到的实体已经被切换为 idle。并且在条件检查系统到本系统之间没有任何其他系统改过 mate 的状态，或杀死 mate。
                let mate_pos = index.get_pos(mate).unwrap();
                movement.direction = (mate_pos - pos.0).normalize();
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
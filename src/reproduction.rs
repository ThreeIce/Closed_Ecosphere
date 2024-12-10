use bevy::prelude::*;
use crate::energy::Energy;
use crate::from_config::from_config;
use crate::movemement::{Movement, MyPosition};
use crate::spatial_index::SpatialIndex;
use crate::type_component::TypeComponent;

pub enum ReproductionState{
    Idle,
    SearchingMate,
    Mating,
    // 处于其它状态，但是可以切换到寻找配偶状态
    Other_can_mate,
    // 处于其它状态，且不能切换到寻找配偶状态
    Other_cant_mate,
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
    pub reproduction_distance: f32,
    // 繁殖所需时间
    pub mating_time: f32,
    _marker: std::marker::PhantomData<T>,
}

pub fn reproduction_state_running<T: ReproductionAgent + TypeComponent, TB: from_config>(
    mut query: Query<(Entity, &mut T, &mut Energy, &mut Movement, &MyPosition)>,
    config: Res<ReproductionConfig<T>>,
    index: Res<SpatialIndex<T>>,
    time: Res<Time>,
    commands: &mut Commands
){
    // 条件切换
    query.par_iter_mut().for_each(|(entity, mut agent, mut energy,_, pos)| {
        match agent.get_state() {
            ReproductionState::Idle => {
                // 切换条件 1：当能量足够时，尝试寻找配偶
                if (energy.0 >= config.energy_threshold) {
                    for mate in index.get_in_radius(pos.0, config.search_radius) {
                        let (_, mut mate_agent, mate_energy,_, _) = query.get(mate).expect("Error in reproduction_state_running, query.get(mate) failed");
                        if mate_energy.0 >= config.energy_threshold {
                            match mate_agent.get_state() {
                                ReproductionState::Idle | ReproductionState::Other_can_mate => {
                                    agent.switch_to_searching_mate(mate);
                                    mate_agent.switch_to_searching_mate(entity);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            },
            ReproductionState::SearchingMate => {
                // 切换条件 1：当配偶死亡或者不处于寻找配偶状态时，切换到空闲状态
                let mate = agent.get_mate().expect("Error in reproduction_state_running, agent.get_mate() failed");
                if let Ok((_, mut mate_agent, _,_, mate_pos)) = query.get(mate) {
                    match mate_agent.get_state() {
                        ReproductionState::SearchingMate => {
                            // 切换条件 2：当与配偶进入繁殖距离，双双进入繁殖状态
                            if pos.0.distance(mate_pos.0) <= config.reproduction_distance {
                                agent.switch_to_mating(config.mating_time);
                                mate_agent.switch_to_mating(config.mating_time);
                            }
                        }
                        _ => {
                            agent.switch_to_idle();
                        }
                    }
                } else {
                    agent.switch_to_idle();
                }

            }
            ReproductionState::Mating => {
                // 切换条件 1：繁殖对象繁殖一半跑了或者死了
                let mate = agent.get_mate().expect("Error in reproduction_state_running, agent.get_mate() failed");
                if let Ok((_, mut mate_agent, mut mate_energy, _, mate_pos)) = query.get_mut(mate) {
                    match mate_agent.get_state() {
                        ReproductionState::Mating => {
                            // 切换条件 2：繁殖时间到，消耗能量并生产孩子
                            let timer = agent.get_reproduction_timer();
                            timer.tick(time.delta());
                            if timer.just_finished() {
                                energy.0 -= config.energy_cost;
                                mate_energy.0 -= config.energy_cost;
                                agent.switch_to_idle();
                                mate_agent.switch_to_idle();
                                commands.spawn(TB::from_config(&config, (mate_pos.0.x + pos.0.x) / 2.0, (mate_pos.0.y + pos.0.y) / 2.0));
                            }
                        }
                        ReproductionState::SearchingMate => {
                            // 这显然是不可能的，正在繁殖的对象只可能突然死了或者跑路了，不可能突然继续找对象
                            panic!("Error in reproduction_state_running, mate is searching mate");
                        }
                        _ => {
                            agent.switch_to_idle();
                        }
                    }
                } else {
                    agent.switch_to_idle();
                }
            }
            _ => {}
        }
    });
    // 状态运行
    query.par_iter_mut().for_each(|(entity, mut agent, mut energy,mut movement, pos)| {
        match agent.get_state() {
            ReproductionState::SearchingMate => {
                // 寻找配偶状态下，不断更新配偶位置
                let mate = agent.get_mate().unwrap();
                // 按理来说如果实体已经被删除了，在上方的代码中就会把状态给切换掉了。
                let (_, _, _, _, mate_pos) = query.get(mate).expect("Error in reproduction_state_running, query.get(mate) failed");
                movement.direction = (mate_pos.0 - pos.0).normalize();
            }
            _ => {}
        }
    });
}
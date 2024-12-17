use std::collections::HashSet;
use bevy::prelude::*;
use crate::energy::Energy;
use crate::health::Health;
use crate::movemement::{Movement, MyPosition};
use crate::spatial_index::SpatialIndex;
use crate::type_component::TypeComponent;

const ATTACK_DISTANCE: f32 = 10.0;

pub trait HunterAgent
{
    fn is_idle(&self) -> bool;
    fn is_hunting(&self) -> bool;
    fn is_attack_cooling(&self) -> bool;
    fn is_eating(&self) -> bool;
    fn switch_to_idle(&mut self);
    fn switch_to_hunting(&mut self, prey: Entity);
    fn switch_back_to_hunting(&mut self);
    fn switch_to_attack_cooling(&mut self, cooling_time: f32);
    fn switch_to_eating(&mut self, energy_gain: f32, eating_time: f32);
    fn get_last_prey_energy_gain(&self) -> f32;
    fn get_prey(&self) -> Option<Entity>;
    fn get_attack_cooling_timer(&mut self) -> &mut Timer;
    fn get_eating_timer(&mut self) -> &mut Timer;
}
#[derive(Resource)]
pub struct Damage<T: TypeComponent>{
    pub damage: f32,
    _marker: std::marker::PhantomData<T>,
}
impl<T> Damage<T> where T: TypeComponent
{
    pub fn new(damage: f32) -> Self
    {
        Damage{
            damage,
            _marker: std::marker::PhantomData
        }
    }
}
#[derive(Resource)]
pub struct EnergyGain<T: TypeComponent>{
    pub energy_gain: f32,
    _marker: std::marker::PhantomData<T>,
}
impl<T> EnergyGain<T> where T: TypeComponent
{
    pub fn new(energy_gain: f32) -> Self
    {
        EnergyGain{
            energy_gain,
            _marker: std::marker::PhantomData
        }
    }
}
#[derive(Resource)]
pub struct AttackCoolingTime<T: TypeComponent>{
    pub time: f32,
    _marker: std::marker::PhantomData<T>,
}
impl<T> AttackCoolingTime<T> where T: TypeComponent
{
    pub fn new(time: f32) -> Self
    {
        AttackCoolingTime{
            time,
            _marker: std::marker::PhantomData
        }
    }
}
#[derive(Resource)]
pub struct EatingTime<T: TypeComponent>{
    pub time: f32,
    _marker: std::marker::PhantomData<T>,
}
impl<T> EatingTime<T> where T: TypeComponent
{
    pub fn new(time: f32) -> Self
    {
        EatingTime{
            time,
            _marker: std::marker::PhantomData
        }
    }
}
pub fn move_to_prey<TH,TP>(mut hunter_query: Query<(&mut TH, &mut Movement, &MyPosition)>,
                           prey_query: Query<(&TP, &MyPosition)>) where TH: Component + HunterAgent, TP: Component + TypeComponent
{
    hunter_query.par_iter_mut().for_each(|(mut hunter_agent, mut movement, hunter_pos)| {
        if hunter_agent.is_hunting()
        {
            if let Ok((_, prey_pos)) = prey_query.get(hunter_agent.get_prey().unwrap())
            {
                movement.direction = (prey_pos.0 - hunter_pos.0).normalize();
            }
            else
            {
                // 猎物可能是已经死亡了，回到 idle
                hunter_agent.switch_to_idle();
            }
        }
    });
}

// State 和 Condition 合并
pub fn on_attack_cooling<TH>(mut hunter_query: Query<(&mut TH, &mut Movement)>, time: Res<Time>) where TH: Component + HunterAgent
{
    hunter_query.par_iter_mut().for_each(|(mut hunter_agent, mut movement)| {
        if hunter_agent.is_attack_cooling() {
            if movement.direction != Vec2::ZERO {
                movement.direction = Vec2::ZERO;
            }
            let timer = hunter_agent.get_attack_cooling_timer();
            timer.tick(time.delta());
            if timer.just_finished()
            {
                hunter_agent.switch_back_to_hunting();
            }
        }
    });
}

pub fn on_eating<TH>(mut hunter_query: Query<(&mut TH, &mut Energy, &mut Movement)>, time: Res<Time>) where TH: Component + HunterAgent
{
    hunter_query.par_iter_mut().for_each(|(mut hunter_agent, mut energy, mut movement)| {
        if hunter_agent.is_eating()
        {
            if movement.direction != Vec2::ZERO {
                movement.direction = Vec2::ZERO;
            }
            let timer = hunter_agent.get_eating_timer();
            timer.tick(time.delta());
            if timer.just_finished()
            {
                energy.0 += hunter_agent.get_last_prey_energy_gain();
                hunter_agent.switch_to_idle();
            }
        }
    });
}

pub fn find_prey<TH,TP>(mut hunter_query:Query<(&mut TH, &MyPosition, &mut Movement)>,
                        index: Res<SpatialIndex<TP>>) where TH: Component + HunterAgent, TP: Component + TypeComponent
{
    hunter_query.par_iter_mut().for_each(|(mut hunter_agent, hunter_pos, mut movement)| {
        if hunter_agent.is_idle()
        {
            // 重置速度
            if movement.direction != Vec2::ZERO {
                movement.direction = Vec2::ZERO;
            }
            if let Some((_, &nearby)) = index.get_nearest(hunter_pos.0){
                hunter_agent.switch_to_hunting(nearby);
            }
        }
    });
}
pub fn attack<TH,TP>(mut hunter_query: Query<(&mut TH, &MyPosition)>,
                     mut prey_query: Query<(&MyPosition, &mut Health),With<TP>>,
                     damage: Res<Damage<TH>>,
                     energy_gain: Res<EnergyGain<TP>>,
                     cooling_time: Res<AttackCoolingTime<TH>>,
                     eating_time: Res<EatingTime<TH>>,
                     mut commands: Commands
) where TH: Component + HunterAgent + TypeComponent, TP: Component + TypeComponent
{
    let mut to_remove = HashSet::<Entity>::new();
    hunter_query.iter_mut().for_each(|(mut hunter_agent, hunter_pos)| {
        if hunter_agent.is_hunting()
        {
            let entity = hunter_agent.get_prey().unwrap();
            if let Ok((prey_pos, mut prey_health)) = prey_query.get_mut(entity)
            {
                // 检测猎物 entity 是否已经被删除
                if !to_remove.contains(&entity) {
                    if hunter_pos.0.distance(prey_pos.0) < ATTACK_DISTANCE
                    {
                        prey_health.0 -= damage.damage;
                        if prey_health.0 <= 0.0
                        {
                            hunter_agent.switch_to_eating(energy_gain.energy_gain, eating_time.time);
                            to_remove.insert(entity);

                        } else {
                            hunter_agent.switch_to_attack_cooling(cooling_time.time);
                        }
                    }
                }
            }
            // 如果猎物不存在了，交由 move_to_prey 捕获并处理，此处不处理
        }
    });
    to_remove.iter().for_each(|e| {
        commands.entity(*e).despawn();
    });
}


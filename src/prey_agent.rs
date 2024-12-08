use bevy::prelude::*;
use crate::config::Config;
use crate::energy::Energy;
use crate::health::Health;
use crate::movemement::{Movement, MyPosition};
use crate::spatial_index::SpatialIndex;
use crate::type_component::TypeComponent;

const ATTACK_DISTANCE: f32 = 1.0;

pub trait HunterState
{
    fn is_idle(&self) -> bool;
    fn is_hunting(&self) -> bool;
    fn is_attack_cooling(&self) -> bool;
    fn is_eating(&self) -> bool;
    fn switch_to_idle(&mut self);
    fn switch_to_hunting(&mut self, prey: Entity);
    fn switch_back_to_hunting(&mut self);
    fn switch_to_attack_cooling(&mut self);
    fn switch_to_eating(&mut self, energy_gain: f32);
    fn get_last_prey_energy_gain(&self) -> f32;
    fn get_prey(&self) -> Entity;
    fn get_attack_cooling_timer(&mut self) -> &mut Timer;
    fn get_eating_timer(&mut self) -> &mut Timer;
}
#[derive(Resource)]
pub struct Damage<T: HunterState>{
    pub damage: f32,
    _marker: std::marker::PhantomData<T>,
}
#[derive(Resource)]
pub struct EnergyGain<T: TypeComponent>{
    pub energy_gain: f32,
    _marker: std::marker::PhantomData<T>,
}
pub fn move_to_prey<TH,TP>(mut hunter_query: Query<(&mut TH, &mut Movement, &MyPosition)>,
                           prey_query: Query<(&TP, &MyPosition)>) where TH: Component + HunterState, TP: Component + TypeComponent
{
    hunter_query.iter_mut().for_each(|(mut hunter_state, mut movement, hunter_pos)| {
        if hunter_state.is_hunting()
        {
            if let Ok((_, prey_pos)) = prey_query.get(hunter_state.get_prey())
            {
                movement.direction = (prey_pos.0 - hunter_pos.0).normalize();
            }
            else
            {
                // 猎物可能是已经死亡了，回到 idle
                hunter_state.switch_to_idle();
            }
        }
    });
}

// State 和 Condition 合并
pub fn on_attack_cooling<TH>(mut hunter_query: Query<(&mut TH)>, time: Res<Time>) where TH: Component + HunterState
{
    hunter_query.par_iter_mut().for_each(|(mut hunter_state)| {
        let mut timer = hunter_state.get_attack_cooling_timer();
        timer.tick(time.delta());
        if timer.just_finished()
        {
            hunter_state.switch_back_to_hunting();
        }
    });
}

pub fn on_eating<TH>(mut hunter_query: Query<(&mut TH, &mut Energy)>, time: Res<Time>) where TH: Component + HunterState
{
    hunter_query.par_iter_mut().for_each(|(mut hunter_state, mut energy)| {
        if hunter_state.is_eating()
        {
            let mut timer = hunter_state.get_eating_timer();
            timer.tick(time.delta());
            if timer.just_finished()
            {
                energy.0 += hunter_state.get_last_prey_energy_gain();
                hunter_state.switch_to_idle();
            }
        }
    });
}

pub fn find_prey<TH,TP>(mut hunter_query:Query<(&mut TH,&MyPosition)>,
                        index: Res<SpatialIndex<TP>>) where TH: Component + HunterState, TP: Component + TypeComponent
{
    hunter_query.iter_mut().for_each(|(mut hunter_state, hunter_pos)| {
        if hunter_state.is_idle()
        {
            if let Some(nearby) = index.get_nearest(hunter_pos.0){
                hunter_state.switch_to_hunting(nearby);
            }
        }
    });
}
pub fn attack<TH,TP>(mut hunter_query: Query<(&mut TH, &MyPosition)>,
                     mut prey_query: Query<(&MyPosition, &mut Health),With<TP>>,
                     damage: Res<Damage<TH>>,
                     energy_gain: Res<EnergyGain<TP>>,
                     mut commands: Commands
) where TH: Component + HunterState, TP: Component + TypeComponent
{
    hunter_query.iter_mut().for_each(|(mut hunter_state, hunter_pos)| {
        if hunter_state.is_hunting()
        {
            if let Ok((prey_pos, mut prey_health)) = prey_query.get_mut(hunter_state.get_prey())
            {
                if hunter_pos.0.distance(prey_pos.0) < ATTACK_DISTANCE
                {
                    prey_health.0 -= damage.damage;
                    if prey_health.0 <= 0.0
                    {
                        hunter_state.switch_to_eating(energy_gain.energy_gain);
                        commands.entity(hunter_state.get_prey()).despawn();
                    }
                    else {
                        hunter_state.switch_to_attack_cooling();
                    }
                }
            }
            // 如果猎物不存在了，交由 move_to_prey 捕获并处理
        }
    });
}


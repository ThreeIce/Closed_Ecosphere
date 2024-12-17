use std::time::Duration;
use crate::spatial_index::SpatialIndex;
use crate::type_component::TypeComponent;
use bevy::prelude::*;
use crate::movemement::{Movement, MyPosition};

pub enum EscapeState{
    Fleeing,
    CanFlee,
    CantFlee
}
pub trait EscapeAgent{
    fn get_state(&self) -> EscapeState;
    fn switch_to_fleeing(&mut self);
    fn switch_to_idle(&mut self);
}
#[derive(Resource)]
pub struct EscapeConfig<T: EscapeAgent + TypeComponent>{
    pub flee_distance: f32,
    pub _marker: std::marker::PhantomData<T>,
}
#[derive(Resource, Deref, DerefMut)]
pub struct EscapeTimer(pub Timer);

impl EscapeTimer{
    pub fn new(update_delta_secs: f32) -> EscapeTimer
    {
        EscapeTimer(Timer::new(Duration::from_secs_f32(update_delta_secs), TimerMode::Repeating))
    }
}
// 该状态机过于简单，条件判断和状态机合并进一个系统里
pub fn escape_from<TP: EscapeAgent + TypeComponent, TH: TypeComponent>(
    mut query: Query<(&mut Movement, &mut TP, &MyPosition)>,
    hunter_index: Res<SpatialIndex<TH>>,
    config: Res<EscapeConfig<TP>>,
    time: Res<Time>,
    mut timer: ResMut<EscapeTimer>
){
    if timer.tick(time.delta()).just_finished() {
        query.par_iter_mut().for_each(|(mut movement, mut agent, pos)| {
            let hunter = hunter_index.get_in_radius(pos.0, config.flee_distance);
            match agent.get_state() {
                EscapeState::CanFlee => {
                    if !hunter.is_empty() {
                        agent.switch_to_fleeing();
                        let hunter_pos = hunter_index.get_pos(*hunter[0].1).unwrap();
                        movement.direction = (pos.0 - hunter_pos).normalize();
                    }
                }
                EscapeState::Fleeing => {
                    if hunter.is_empty() {
                        agent.switch_to_idle();
                        movement.direction = Vec2::ZERO;
                    } else {
                        let hunter_pos = hunter_index.get_pos(*hunter[0].1).unwrap();
                        movement.direction = (pos.0 - hunter_pos).normalize();
                    }
                }
                EscapeState::CantFlee => {}
            }
        });
    }
}
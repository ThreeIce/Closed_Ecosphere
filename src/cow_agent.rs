use bevy::prelude::{Component, Entity, Timer, TimerMode};
use crate::prey_agent::HunterAgent;
use crate::reproduction::{ReproductionAgent, ReproductionState};
use crate::type_component::TypeComponent;

pub enum CowState
{
    Idle,
    Hunting,
    AttackCooling,
    Eating,
    SearchingMate,
    Mating,
}
#[derive(Component)]
pub struct CowAgent
{
    pub state: CowState,
    pub timer: Timer,
    pub target: Option<Entity>,
    pub last_energy_gain: f32,
}
impl TypeComponent for CowAgent{

}
impl HunterAgent for CowAgent{
    fn is_idle(&self) -> bool {
        match self.state {
            CowState::Idle => true,
            _ => false
        }
    }
    fn is_hunting(&self) -> bool {
        match self.state {
            CowState::Hunting => true,
            _ => false
        }
    }

    fn is_attack_cooling(&self) -> bool {
        match self.state {
            CowState::AttackCooling => true,
            _ => false
        }
    }

    fn is_eating(&self) -> bool {
        match self.state {
            CowState::Eating => true,
            _ => false
        }
    }

    fn switch_to_idle(&mut self) {
        self.state = CowState::Idle;
    }

    fn switch_to_hunting(&mut self, prey: Entity) {
        self.state = CowState::Hunting;
        self.target = Some(prey);
    }

    fn switch_back_to_hunting(&mut self) {
        self.state = CowState::Hunting;
    }

    fn switch_to_attack_cooling(&mut self, cooling_time: f32) {
        self.state = CowState::AttackCooling;
        self.timer = Timer::from_seconds(cooling_time, TimerMode::Once);
    }

    fn switch_to_eating(&mut self, energy_gain: f32, eating_time: f32) {
        self.state = CowState::Eating;
        self.timer = Timer::from_seconds(eating_time, TimerMode::Once);
        self.last_energy_gain = energy_gain;
    }

    fn get_last_prey_energy_gain(&self) -> f32 {
        self.last_energy_gain
    }

    fn get_prey(&self) -> Option<Entity> {
        self.target
    }

    fn get_attack_cooling_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }

    fn get_eating_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }
}

impl ReproductionAgent for CowAgent{
    fn get_state(&self) -> ReproductionState {
        match self.state {
            CowState::Idle => ReproductionState::Idle,
            CowState::SearchingMate => ReproductionState::SearchingMate,
            CowState::Mating => ReproductionState::Mating,
            CowState::Hunting => ReproductionState::OtherCanMate,
            _ => ReproductionState::OtherCantMate
        }
    }

    fn switch_to_idle(&mut self) {
        self.state = CowState::Idle;
    }

    fn switch_to_searching_mate(&mut self, mate: Entity) {
        self.state = CowState::SearchingMate;
        self.target = Some(mate);
    }

    fn switch_to_mating(&mut self, mating_time: f32) {
        self.state = CowState::Mating;
        self.timer = Timer::from_seconds(mating_time, TimerMode::Once);
    }

    fn get_mate(&self) -> Option<Entity> {
        self.target
    }

    fn get_reproduction_timer(&mut self) -> &mut Timer {
        &mut self.timer
    }
}
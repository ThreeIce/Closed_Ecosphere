use bevy::prelude::*;
use crate::movemement::{Movement, MyPosition};
use crate::spatial_index::SpatialIndex;
use crate::type_component::TypeComponent;

pub trait HunterState
{
    fn is_idle(&self) -> bool;
    fn is_hunting(&self) -> bool;
    fn is_attack_cooling(&self) -> bool;
    fn is_eating(&self) -> bool;
    fn switch_to_idle(&mut self);
    fn switch_to_hunting(&mut self, prey: Entity);
    fn switch_to_attack_cooling(&mut self);
    fn switch_to_eating(&mut self);
    fn get_prey(&self) -> Entity;
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

pub fn move_to_prey<TH,TP>(mut hunter_query: Query<(&TH, &mut Movement, &MyPosition)>,
                           prey_query: Query<(&TP, &MyPosition)>) where TH: Component + HunterState, TP: Component + TypeComponent
{
    hunter_query.iter_mut().for_each(|(hunter_state, mut movement, hunter_pos)| {
        if hunter_state.is_hunting()
        {
            if let Ok((_, prey_pos)) = prey_query.get(hunter_state.get_prey())
            {
                movement.direction = (prey_pos.0 - hunter_pos.0).normalize();
            }
        }
    });
}
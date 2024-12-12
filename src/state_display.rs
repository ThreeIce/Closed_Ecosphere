use bevy::prelude::*;
use crate::config::Config;
use crate::cow_agent::{CowAgent, CowState};
use crate::tiger_agent::{TigerAgent, TigerState};

pub fn cow_state_display(
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>,&CowAgent)>,
    config: Res<Config>,
) {
    query.par_iter_mut().for_each(|(mut mesh_material2d,cow_agent)| {
        match cow_agent.state {
            CowState::Idle => {
                mesh_material2d.0 = config.idle_color.clone();
            }
            CowState::Hunting => {
                mesh_material2d.0 = config.hunting_color.clone();
            }
            CowState::AttackCooling => {
                mesh_material2d.0 = config.attack_cooling_color.clone();
            }
            CowState::Eating => {
                mesh_material2d.0 = config.eating_color.clone();
            }
            CowState::SearchingMate => {
                mesh_material2d.0 = config.searching_mate_color.clone();
            }
            CowState::Mating => {
                mesh_material2d.0 = config.mating_color.clone();
            }
            CowState::Fleeing => {
                mesh_material2d.0 = config.escaping_color.clone();
            }
        }
    });
}
pub fn tiger_state_display(
    mut query: Query<(&mut MeshMaterial2d<ColorMaterial>,&TigerAgent)>,
    config: Res<Config>,
) {
    query.par_iter_mut().for_each(|(mut mesh_material2d,tiger_agent)| {
        match tiger_agent.state {
            TigerState::Idle => {
                mesh_material2d.0 = config.idle_color.clone();
            }
            TigerState::Hunting => {
                mesh_material2d.0 = config.hunting_color.clone();
            }
            TigerState::AttackCooling => {
                mesh_material2d.0 = config.attack_cooling_color.clone();
            }
            TigerState::Eating => {
                mesh_material2d.0 = config.eating_color.clone();
            }
            TigerState::SearchingMate => {
                mesh_material2d.0 = config.searching_mate_color.clone();
            }
            TigerState::Mating => {
                mesh_material2d.0 = config.mating_color.clone();
            }
        }
    });
}
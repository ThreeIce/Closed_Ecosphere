use bevy::prelude::{Bundle, Res};
use crate::config::Config;

pub trait from_config: Bundle{
    fn from_config(config: &Res<Config>, x: f32, y: f32) -> Self;
}
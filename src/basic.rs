use bevy::prelude::*;
use bevy::utils::{HashMap, HashSet};

#[derive(Component)]
pub struct Health(pub f32);

// 标签组件
#[derive(Component)]
pub struct Grass;
#[derive(Component)]
pub struct Cow;
#[derive(Component)]
pub struct Tiger;

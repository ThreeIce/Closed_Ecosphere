use bevy::prelude::*;
use crate::basic::*;
use crate::bundle::*;
// 草的繁殖

// 繁殖间隔计时器
#[derive(Component, Deref, DerefMut)]
pub struct GrassReproductionTimer(Timer);
#[derive(Component, Deref, DerefMut)]
pub struct GrassNeighborCount(pub usize);
impl GrassReproductionTimer {
    pub fn from_reproduction_delta(delta: f32) -> Self {
        GrassReproductionTimer(Timer::from_seconds(delta, TimerMode::Repeating))
    }
}
pub fn on_grass_die(
    trigger: Trigger<OnRemove, GrassNeighborCount>,
    mut query: Query<(Entity, &Transform,&mut GrassNeighborCount)>,
    mut index: ResMut<SpatialIndex<Grass>>,
    config: Res<Config>
){
    let pos = query.get(trigger.entity()).unwrap().1.translation.xy();
    index.remove(pos, trigger.entity());
    index.get_in_radius(pos, config.grass_reproduction_radius,
                        query.transmute_lens::<&Transform>().query())
        .iter().for_each(|e|{
        if let Ok((_,_,mut count)) = query.get_mut(*e){
            count.0 -= 1;
        }else {
            error!("Error in on_grass_die, query.get_mut(*e) failed");
        }
    });
}
pub fn on_grass_birth(
    trigger: Trigger<OnAdd, GrassNeighborCount>,
    mut query: Query<(Entity, &Transform,&mut GrassNeighborCount)>,
    mut index: ResMut<SpatialIndex<Grass>>,
    config: Res<Config>
){
    let pos = query.get(trigger.entity()).unwrap().1.translation.xy();
    let neighbor = index.get_in_radius(pos, config.grass_reproduction_radius,
                                       query.transmute_lens::<&Transform>().query());
    neighbor.iter().for_each(|e|{
        if let Ok((_,_,mut count)) = query.get_mut(*e){
            count.0 += 1;
        }else {
            println!("Error in on_grass_birth, query.get_mut(*e) failed");
            error!("Error in on_grass_birth, query.get_mut(*e) failed");
        }
    });
    if let Ok((_,_,mut count)) = query.get_mut(trigger.entity()){
        count.0 = neighbor.len();
    }else {
        println!("Error in on_grass_birth, query.get_mut(trigger.entity()) failed");
        error!("Error in on_grass_birth, query.get_mut(trigger.entity()) failed");
    }
    index.insert(pos, trigger.entity());
}
// 繁殖系统
// 如果草周边草的数量小于 3，草会以一阶段概率繁殖，
// 如果草周边草的数量为 3-6，它会以二阶段概率繁殖，
// 如果草周边草的数量 ≥ 7，草将停止繁殖。
pub fn grass_reproduction_system(time: Res<Time>,
                                 config: Res<Config>,
                                 mut query: Query<(&mut GrassReproductionTimer,
                                                   &GrassNeighborCount,
                                                   &Transform)>,
                                 mut commands: Commands,
){
    for (mut timer, count, transform) in query.iter_mut(){
        if timer.tick(time.delta()).just_finished(){
            let seed = rand::random::<f32>();
            if (count.0 < 3 && seed < config.grass_reproduction_rate_1)
                || (count.0 >= 3 && count.0 <= 6 && seed < config.grass_reproduction_rate_2){
                // 在生成范围内随机选一个点作为生成坐标
                let x = transform.translation.x + rand::random::<f32>() * 2.0 * config.grass_reproduction_radius - config.grass_reproduction_radius;
                let y = transform.translation.y + rand::random::<f32>() * 2.0 * config.grass_reproduction_radius - config.grass_reproduction_radius;
                commands.spawn(GrassBundle::from_config(&config, x, y));
            }
        }
    }
}

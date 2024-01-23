use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::Query,
    },
    math::Vec2,
    prelude::{Children, Rect},
    transform::components::{Transform, GlobalTransform},
    utils::{HashMap, HashSet},
};

use crate::{
    debug_bounding_box::{get_transform, Boundry, WallType},
    Floor, Player,
};

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub collider_entities: HashSet<Entity>,
    pub is_grounded: bool,
    pub rect: Rect,
    pub offset: Vec2,
}

impl Collider {
    pub fn new(radius: f32, rect: Rect, offset: Vec2) -> Self {
        Self {
            radius,
            collider_entities: HashSet::new(),
            is_grounded: false,
            rect,
            offset,
        }
    }
}

pub fn floor_collision(
    mut player: Query<(&Player, &mut Collider, &Transform, &Children)>,
    floor_query: Query<(&Floor, &Collider, &Transform, &Children), Without<Player>>,
    children: Query<(&Boundry, &GlobalTransform)>,
) {
    for (_, mut player_collider, _, p_children) in &mut player {
        player_collider.collider_entities.clear();

        let mut p_boundry_map = HashMap::new();

        for &i in p_children {
            if let Ok((c_boundry, c_trans)) = children.get(i) {
                p_boundry_map.insert(c_boundry.wall_type.clone(), c_trans);
            }
        }

        for (_, _, _, f_children) in &floor_query {
            let mut f_boundry_map = HashMap::new();

            for &i in f_children {
                if let Ok((f_boundry, f_trans)) = children.get(i) {
                    f_boundry_map.insert(f_boundry.wall_type.clone(), f_trans);
                }
            }

            let Some(p_bottom) = p_boundry_map.get(&WallType::Bottom) else {
                continue;
            };

            let Some(p_left) = p_boundry_map.get(&WallType::Left) else {
                continue;
            };

            let Some(p_right) = p_boundry_map.get(&WallType::Right) else {
                continue;
            };

            let Some(f_top) = f_boundry_map.get(&WallType::Top) else {
                continue;
            };

            let Some(f_left) = f_boundry_map.get(&WallType::Left) else {
                continue;
            };

            let Some(f_right) = f_boundry_map.get(&WallType::Right) else {
                continue;
            };

            
            if p_right.translation().x > f_left.translation().x && p_left.translation().x < f_right.translation().x && (p_bottom.translation().y - 20.) <= f_top.translation().y {
                player_collider.is_grounded = true;
            } else {
                player_collider.is_grounded = false;
            }

            // if colliding left 
             
            // if colliding top 
            
            // if colliding right
        }
    }
}

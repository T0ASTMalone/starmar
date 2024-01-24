use bevy::{
    ecs::{component::Component, query::Without, system::Query},
    math::Vec2,
    prelude::{Children, Rect},
    transform::components::{GlobalTransform, Transform},
    utils::HashMap,
};

use crate::{
    debug_bounding_box::{Boundry, WallType},
    Floor, Player,
};

pub struct CollidingSides {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl CollidingSides {
    pub fn new() -> Self {
        Self {
            top: false,
            bottom: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub is_grounded: bool,
    pub is_colliding: CollidingSides,
    pub rect: Rect,
    pub offset: Vec2,
}

impl Collider {
    pub fn new(radius: f32, rect: Rect, offset: Vec2) -> Self {
        Self {
            radius,
            is_grounded: false,
            rect,
            offset,
            is_colliding: CollidingSides::new(),
        }
    }
}

pub fn floor_collision(
    mut player: Query<(&Player, &mut Collider, &Transform, &Children)>,
    floor_query: Query<(&Floor, &Collider, &Transform, &Children), Without<Player>>,
    children: Query<(&Boundry, &GlobalTransform)>,
) {
    for (_, mut player_collider, _, p_children) in &mut player {
        // clear collided sides
        player_collider.is_colliding = CollidingSides::new();

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

            let Some(p_top) = p_boundry_map.get(&WallType::Top) else {
                continue;
            };

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

            let Some(f_bottom) = f_boundry_map.get(&WallType::Bottom) else {
                continue;
            };

            let Some(f_left) = f_boundry_map.get(&WallType::Left) else {
                continue;
            };

            let Some(f_right) = f_boundry_map.get(&WallType::Right) else {
                continue;
            };

            if p_right.translation().x > f_left.translation().x
                && p_left.translation().x < f_right.translation().x
                && !player_collider.is_colliding.bottom
            {
                if (p_bottom.translation().y - 20.) <= f_top.translation().y {
                    player_collider.is_colliding.bottom = true;
                } else {
                    player_collider.is_colliding.bottom = false;
                }
            }

            if p_right.translation().x > f_left.translation().x
                && p_left.translation().x < f_right.translation().x
                && !player_collider.is_colliding.top
            {
                if p_top.translation().y >= f_bottom.translation().y {
                    player_collider.is_colliding.top = true;
                } else {
                    player_collider.is_colliding.top = false;
                }
            }

            // if colliding right
            if p_top.translation().y > f_bottom.translation().y
                && p_bottom.translation().y < f_top.translation().y
                && !player_collider.is_colliding.right
            {
                if p_right.translation().x <= f_left.translation().x {
                    player_collider.is_colliding.right = true;
                } else {
                    player_collider.is_colliding.right = false;
                }
            }

            // if colliding left
            if p_top.translation().y > f_bottom.translation().y
                && p_bottom.translation().y < f_top.translation().y
                && !player_collider.is_colliding.left
            {
                if p_left.translation().x >= f_right.translation().x {
                    player_collider.is_colliding.left = true;
                } else {
                    player_collider.is_colliding.left = false;
                }
            }
        }
    }
}

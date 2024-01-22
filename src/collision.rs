use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::Query,
    },
    math::Vec2,
    transform::components::Transform,
    utils::HashSet, prelude::{Rect, Children},
};

use crate::{Floor, Player, debug_bounding_box::{get_transform, WallType, Boundry}};

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub collider_entities: HashSet<Entity>,
    pub is_grounded: bool,
    pub rect: Rect,
    pub offset: Vec2
}

impl Collider {
    pub fn new(radius: f32, rect: Rect, offset: Vec2) -> Self {
        Self {
            radius,
            collider_entities: HashSet::new(),
            is_grounded: false,
            rect,
            offset
        }
    }
}

pub fn floor_collision(
    mut player: Query<(&Player, &mut Collider, &Transform, &Children)>,
    floor_query: Query<(&Floor, &Collider, &Transform, &Children), Without<Player>>,
    children: Query<(&Boundry, &Transform)>
) {
    for (_, mut player_collider, player_transform, p_children) in &mut player {
        player_collider.collider_entities.clear();

        // get bottom bar y for player
        // get top bar y for player
        // get left bar x for player
        // get right bar x for player
        let p_t_trans = get_transform(player_transform, -player_collider.offset.x, (player_collider.rect.height() / 2.) - player_collider.offset.y);
        let p_b_trans = get_transform(player_transform, -player_collider.offset.x, -(player_collider.rect.height() / 2.) - player_collider.offset.y);
        let p_r_trans = get_transform(player_transform, (player_collider.rect.width() / 2.) - player_collider.offset.x, -player_collider.offset.y);
        let p_l_trans = get_transform(player_transform, -(player_collider.rect.width() / 2.) - player_collider.offset.x, -player_collider.offset.y);

        for &i in p_children {
            // loop throught floor 
            // if x and y interset break
            println!("{:?}", children.get(i));
        }

        for (_, floor_collider, transform, _) in &floor_query {
            // get top bar y for floor 
            // get right bar x for floor
            // get left bar x for floor
            let f_t_trans = get_transform(transform, -floor_collider.offset.x, (floor_collider.rect.height() / 2.) - floor_collider.offset.y);
            let f_b_trans = get_transform(transform, -floor_collider.offset.x, -(floor_collider.rect.height() / 2.) - floor_collider.offset.y);
            let f_r_trans = get_transform(transform, (floor_collider.rect.width() / 2.) - floor_collider.offset.x, -floor_collider.offset.y);
            let f_l_trans = get_transform(transform, -(floor_collider.rect.width() / 2.) - floor_collider.offset.x, -floor_collider.offset.y);
            

            // (p_right.x > f_left.x  || p_left.x < f_right.x) // player is in floor in the x
            // && p_bottom.y <= f.top.y // player is ontop of floor
            if p_r_trans.translation.x > f_l_trans.translation.x && p_l_trans.translation.x < f_r_trans.translation.x && p_b_trans.translation.y <= f_t_trans.translation.y {
                // println!("player collided with top of floor {} {}", p_b_trans.translation.y, f_t_trans.translation.y);
            } else {
                //println!("nope");
            }
            
            let distance = transform.translation.distance(player_transform.translation);

            let player_distance = player_collider.radius / 2.;
            let floor_distance = floor_collider.radius / 2.;

            if distance <= player_distance + floor_distance {
                // println!("player collided with top of floor");
                player_collider.is_grounded = true;
            }
        }
    }
}

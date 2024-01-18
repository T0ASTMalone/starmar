use bevy::{ecs::{component::Component, system::Query, entity::Entity, query::{With, Without}}, math::Vec2, utils::HashSet, transform::components::Transform};

use crate::{Player, Floor};


#[derive(Component)]
pub struct Collider {
    pub radius: f32,
    pub collider_entities: HashSet<Entity>,
    pub is_grounded: bool
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            collider_entities: HashSet::new(),
            is_grounded: false 
        }
    }
}

pub fn floor_collision(mut player: Query<(&Player, &mut Collider, &Transform)>, floor_query: Query<(&Floor, &Collider, &Transform), Without<Player>>) {
   for (_, mut player_collider, player_transform) in &mut player {

       player_collider.collider_entities.clear();

       for (_, floor_collider, transform) in &floor_query {
            let distance = transform.translation.distance(player_transform.translation);

            let player_distance = player_collider.radius / 2.; 
            let floor_distance = floor_collider.radius / 2.; 

            if distance <= player_distance + floor_distance {
               player_collider.is_grounded = true;
            }
       }
   }
}

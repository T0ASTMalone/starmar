use bevy::{ecs::{system::Query, component::Component}, transform::components::Transform};

use crate::collision::Collider;

#[derive(Component)]
pub struct Gravity;

const GRAVITY: f32 = 9.8;

pub fn gravity_system(mut query: Query<(&Gravity, &mut Transform, &Collider)>) {
    for (_, mut transform, collider) in &mut query {
        // if not colliding with floor
        if !collider.is_grounded {
            transform.translation.y -= GRAVITY;
        }
        
    }
}

use bevy::{ecs::{system::Query, component::Component, query::With}, transform::components::Transform};

use crate::collision::Collider;

#[derive(Component)]
pub struct Gravity;

const GRAVITY: f32 = 9.8;

pub fn gravity_system(mut query: Query<(&mut Transform, &Collider), With<Gravity>>) {
    for (mut transform, collider) in &mut query {
        if !collider.is_colliding.bottom {
            transform.translation.y -= GRAVITY;
        }
    }
}

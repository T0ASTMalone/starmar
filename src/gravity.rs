use bevy::{ecs::{system::Query, component::Component}, transform::components::Transform};

#[derive(Component)]
pub struct Gravity;

const GRAVITY: f32 = 9.8;

pub fn gravity_system(mut query: Query<(&Gravity, &mut Transform)>) {
    for (_, mut transform) in &mut query {
        // if not colliding with floor
        transform.translation.y -= GRAVITY;
    }
}

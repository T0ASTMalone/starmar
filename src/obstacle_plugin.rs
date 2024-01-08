use bevy::prelude::*;

use crate::{Floor, collision_detection::Collider};

const RADIUS: f32 = 30.;

#[derive(Component, Debug, Clone)]
pub struct Obstacle {
    pub dimensions: Vec2,
}

impl Obstacle {
    pub fn new(dimensions: Vec2) -> Self {
        Self { dimensions }
    }
}

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_obstacles);
    }
} 

fn spawn_obstacles(mut commands: Commands) {
    let obstacle = Obstacle::new(Vec2::splat(RADIUS * 2.));
    commands.spawn(
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.25),
                    custom_size: Some(obstacle.dimensions),
                    ..default()
                },
                transform: Transform::from_xyz(100., -66., 0.),
                ..default()
            }, 
            obstacle.clone(),
            Floor,
            Collider::new(RADIUS) 
        )
    );

    commands.spawn(
        (
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.25, 0.25, 0.25),
                    custom_size: Some(obstacle.dimensions),
                    ..default()
                },
                transform: Transform::from_xyz(300., -66., 0.),
                ..default()
            }, 
            obstacle.clone(),
            Floor,
            Collider::new(RADIUS) 
        )
    );
}

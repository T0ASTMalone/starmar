use bevy::{
    ecs::{
        component::Component,
        system::{Commands, Query},
    },
    math::{Rect, Vec2},
    prelude::default,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

#[derive(Component, Debug)]
pub struct DebugBoundingBox {
    rect: Rect,
    name: String 
}

impl DebugBoundingBox {
    pub fn new(rect: Rect, name: String) -> Self {
        Self { rect, name }
    }
}

const WIDTH: f32 = 1.;
const BOUNDING_Z: f32 = 10.;

pub fn draw_bounding_boxes(mut commands: Commands, query: Query<(&DebugBoundingBox, &Transform)>) {
    for (dbg_bouding_box, transform) in &query {
        let top = Color::rgb(1., 1., 1.); // white
        let bottom = Color::rgb(0., 0., 0.); // black 
        let left = Color::rgb(0.5, 0.5, 0.75); // blue? 

        let half_height = dbg_bouding_box.rect.height() / 2.;
        let half_width = dbg_bouding_box.rect.width() / 2.;

        if dbg_bouding_box.name == "player" {
            // println!("scale x : {:?}", transform.scale.x);
        }

        let mut top_bar = transform.clone();
        top_bar.translation.y += half_height * transform.scale.y;
        top_bar.translation.z = BOUNDING_Z;

        let mut bottom_bar = transform.clone();
        bottom_bar.translation.y -= half_height * transform.scale.y;
        bottom_bar.translation.z = BOUNDING_Z;

        let mut left_bar = transform.clone();
        left_bar.translation.x -= half_width * transform.scale.x;
        left_bar.translation.z = BOUNDING_Z;

        let mut right_bar = transform.clone();
        right_bar.translation.x += half_width * transform.scale.x;
        right_bar.translation.z = BOUNDING_Z;

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: top,
                custom_size: Some(Vec2::new(dbg_bouding_box.rect.width() + (WIDTH * 2.), WIDTH)),
                ..default()
            },
            transform: top_bar,
            ..default()
        });

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: bottom,
                custom_size: Some(Vec2::new(dbg_bouding_box.rect.width() + (WIDTH * 2.), WIDTH)),
                ..default()
            },
            transform: bottom_bar,
            ..default()
        });

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: left,
                custom_size: Some(Vec2::new(WIDTH, dbg_bouding_box.rect.height() + (WIDTH * 2.))),
                ..default()
            },
            transform: left_bar,
            ..default()
        });

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1., 1., 1.),
                custom_size: Some(Vec2::new(WIDTH, dbg_bouding_box.rect.height() + (WIDTH * 2.))),
                ..default()
            },
            transform: right_bar,
            ..default()
        });
    }
}

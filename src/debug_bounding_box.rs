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
}

impl DebugBoundingBox {
    pub fn new(rect: Rect) -> Self {
        Self { rect }
    }
}

const WIDTH: f32 = 0.5;

pub fn draw_bounding_boxes(mut commands: Commands, query: Query<(&DebugBoundingBox, &Transform)>) {
    for (dbg_bouding_box, transform) in &query {
        println!("width: {}", dbg_bouding_box.rect.width());
        let transform = Transform::from_scale(transform.scale);

        let mut top_bar = transform.clone();
        top_bar.translation.y += WIDTH;

        let mut bottom_bar = transform.clone();
        bottom_bar.translation.y -= (dbg_bouding_box.rect.height() + WIDTH) * transform.scale.y;

        let mut left_bar = transform.clone();
        left_bar.translation.x -= (dbg_bouding_box.rect.width() + WIDTH) * (transform.scale.x / 2.);
        left_bar.translation.y -= (dbg_bouding_box.rect.height() + WIDTH) * (transform.scale.y / 2.);

        let mut right_bar = transform.clone();
        right_bar.translation.x += (dbg_bouding_box.rect.width() + WIDTH) * (transform.scale.x / 2.);
        right_bar.translation.y -= (dbg_bouding_box.rect.height() + WIDTH) * (transform.scale.y / 2.);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(dbg_bouding_box.rect.width() + (WIDTH * 2.), WIDTH)),
                ..default()
            },
            transform: top_bar,
            ..default()
        });

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.75),
                custom_size: Some(Vec2::new(dbg_bouding_box.rect.width() + (WIDTH * 2.), WIDTH)),
                ..default()
            },
            transform: bottom_bar,
            ..default()
        });

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(WIDTH, dbg_bouding_box.rect.height() + (WIDTH * 2.))),
                ..default()
            },
            transform: left_bar,
            ..default()
        });

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.75, 0.75),
                custom_size: Some(Vec2::new(WIDTH, dbg_bouding_box.rect.height() + (WIDTH * 2.))),
                ..default()
            },
            transform: right_bar,
            ..default()
        });
    }
}

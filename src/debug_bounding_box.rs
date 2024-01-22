use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query},
    },
    hierarchy::BuildChildren,
    math::{Rect, Vec2},
    prelude::default,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

#[derive(Component, Debug)]
pub struct DebugBoundingBox {
    rect: Rect,
    offset: Vec2,
    name: String,
}

impl DebugBoundingBox {
    pub fn new(rect: Rect, name: String, offset: Vec2) -> Self {
        Self { rect, name, offset }
    }
}

#[derive(Debug)]
pub enum Wall {
    Left,
    Right,
    Top,
    Bottom,
}

const WIDTH: f32 = 1.;
const BOUNDING_Z: f32 = 10.;

const WHITE: Color = Color::rgb(1., 1., 1.);
const GREEN: Color = Color::rgb(0., 1., 0.);
const BLUE: Color = Color::rgb(0., 0., 1.);
const RED: Color = Color::rgb(1., 0., 0.);

pub fn get_transform(transform: &Transform, x: f32, y: f32) -> Transform {
    let mut bar = transform.clone();
    bar.translation.x = x;
    bar.translation.y = y;
    bar.translation.z = BOUNDING_Z;
    bar.scale.x = 1.;
    return bar;
}


pub fn spawn(commands: &mut Commands, entity: Entity, x: f32, y: f32, transform: Transform) {
    let id = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: WHITE,
                custom_size: Some(Vec2::new(
                    // (b.rect.width() + (WIDTH / transform.scale.x)) / transform.scale.x,
                    x,
                    // WIDTH / transform.scale.x,
                    y
                )),
                ..default()
            },
            transform,
            ..default()
        })
        .id();
    commands.entity(entity).add_child(id);
}

pub fn vertical_bar(commands: &mut Commands, transform: &Transform, width: f32, b: &DebugBoundingBox, entity: Entity) {
    let mut bar = transform.clone();
    bar.translation.x = width  - b.offset.x;
    bar.translation.y = 0. - b.offset.y;
    bar.translation.z = BOUNDING_Z;
    bar.scale.x = 1.;

    spawn(
        commands, 
        entity, 
        WIDTH / transform.scale.x,
        (b.rect.height() + (WIDTH / transform.scale.y)) / transform.scale.x,
        bar
    );
}

pub fn horizontal_bar(commands: &mut Commands, transform: &Transform, height: f32, b: &DebugBoundingBox, entity: Entity) {
    let transform = get_transform(transform, -b.offset.x, height - b.offset.y);

    let mut bar = transform.clone();
    bar.translation.y = height - b.offset.y;
    bar.translation.x = -b.offset.x;
    bar.translation.z = BOUNDING_Z;
    bar.scale.y = 1.;

    spawn(
        commands, 
        entity, 
        (b.rect.width() + (WIDTH / transform.scale.x)) / transform.scale.x, 
        WIDTH / transform.scale.x,
        bar
    );
}

pub fn draw_bounding_boxes(
    mut commands: Commands,
    query: Query<(Entity, &DebugBoundingBox, &Transform)>,
) {
    for (entity, dbg_bouding_box, transform) in &query {

        let half_height = dbg_bouding_box.rect.height() / 2.;
        let half_width = dbg_bouding_box.rect.width() / 2.;

        horizontal_bar(&mut commands, transform, half_height, dbg_bouding_box, entity);
        horizontal_bar(&mut commands, transform, -half_height, dbg_bouding_box, entity);
        vertical_bar(&mut commands, transform, half_width, dbg_bouding_box, entity);
        vertical_bar(&mut commands, transform, -half_width, dbg_bouding_box, entity);
    }
}

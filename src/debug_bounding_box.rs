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
pub enum WallType {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Component, Debug)]
pub struct Boundry {
    wall_type: WallType
}

impl Boundry {
    pub fn new(wall_type: WallType) -> Self {
        Self {
            wall_type
        }
    }
}

const WIDTH: f32 = 1.;
const BOUNDING_Z: f32 = 10.;

const WHITE: Color = Color::rgb(1., 1., 1.);

pub fn get_transform(transform: &Transform, x: f32, y: f32) -> Transform {
    let mut bar = transform.clone();
    bar.translation.x = x;
    bar.translation.y = y;
    bar.translation.z = BOUNDING_Z;
    return bar;
}

pub fn spawn(commands: &mut Commands, entity: Entity, x: f32, y: f32, transform: Transform, wall_type: WallType) {
    let id = commands
        .spawn((SpriteBundle {
            sprite: Sprite {
                color: WHITE,
                custom_size: Some(Vec2::new(x, y)),
                ..default()
            },
            transform,
            ..default()
        }, Boundry::new(wall_type)))
        .id();
    commands.entity(entity).add_child(id);
}

pub fn vertical_bar(
    commands: &mut Commands,
    transform: &Transform,
    width: f32,
    b: &DebugBoundingBox,
    entity: Entity,
    wall_type: WallType
) {
    let mut trans = get_transform(transform, width - b.offset.x, -b.offset.y);
    trans.scale.x = 1.;

    spawn(
        commands,
        entity,
        // 1. / transform.scale.x
        WIDTH / transform.scale.x,
        // rect height + (1. / parent.scale.y) / parent.scale.y
        (b.rect.height() + (WIDTH / transform.scale.y)) / transform.scale.y,
        trans,
        wall_type
    );
}

pub fn horizontal_bar(
    commands: &mut Commands,
    transform: &Transform,
    height: f32,
    b: &DebugBoundingBox,
    entity: Entity,
    wall_type: WallType
) {
    let mut trans = get_transform(transform, -b.offset.x, height - b.offset.y);
    trans.scale.y = 1.;
    spawn(
        commands,
        entity,
        (b.rect.width() + (WIDTH / transform.scale.x)) / transform.scale.x,
        WIDTH / transform.scale.y,
        trans,
        wall_type
    );
}

pub fn draw_bounding_boxes(
    mut commands: Commands,
    query: Query<(Entity, &DebugBoundingBox, &Transform)>,
) {
    for (entity, dbg_bouding_box, transform) in &query {
        let half_height = dbg_bouding_box.rect.height() / 2.;
        let half_width = dbg_bouding_box.rect.width() / 2.;

        // top bar
        horizontal_bar(
            &mut commands,
            transform,
            half_height,
            dbg_bouding_box,
            entity,
            WallType::Top
        );

        // bottom bar
        horizontal_bar(
            &mut commands,
            transform,
            -half_height,
            dbg_bouding_box,
            entity,
            WallType::Bottom
        );

        // right bar
        vertical_bar(
            &mut commands,
            transform,
            half_width,
            dbg_bouding_box,
            entity,
            WallType::Right
        );

        // left bar
        vertical_bar(
            &mut commands,
            transform,
            -half_width,
            dbg_bouding_box,
            entity,
            WallType::Left
        );
    }
}

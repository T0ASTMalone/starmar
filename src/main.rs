use std::collections::HashMap;

use bevy::prelude::*;
use collision::Collider;
use controlls::just_pressed_wasd;
use debug_bounding_box::{draw_bounding_boxes, DebugBoundingBox};
use gravity::Gravity;
use sprite_animation_keys::{AnimationActions, AnimationInfo};

use crate::sprite_animation_keys::CAT_MAP;

// modules
pub mod collision;
pub mod controlls;
pub mod debug_bounding_box;
pub mod gravity;
pub mod sprite_animation_keys;

const DEBUG_ANIMATION: AnimationActions = AnimationActions::Idle;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct AnimationMap(HashMap<AnimationActions, AnimationInfo>);

#[derive(Component)]
pub struct CurrentAnimation {
    current_animation: AnimationActions,
    current_animation_idx: usize,
    animation_indeces: Vec<usize>,
    is_loop: bool,
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
    pub prev: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3, prev: Vec3) -> Self {
        Self { value, prev }
    }
}

#[derive(Component)]
pub struct Floor;

#[derive(Resource)]
pub struct World {
   pub pos: Vec2,
}

fn _animation_test(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut CurrentAnimation,
        &AnimationMap,
        &mut TextureAtlasSprite,
    )>,
) {
    if keys.just_pressed(KeyCode::A) {
        for (_, _, _) in &mut query {
            /*
            current_animation.current_animation =
                if current_animation.current_animation >= map.0.len() - 1 {
                    0
                } else {
                    current_animation.current_animation + 1
                };

            let indeces = map
                .0
                .get(&current_animation.current_animation)
                .unwrap()
                .clone();

            current_animation.animation_indeces = indeces;
            current_animation.current_animation_idx = 0;
            */
        }
    } else if keys.just_pressed(KeyCode::S) {
        for (mut animation, _, mut sprite) in &mut query {
            animation.current_animation_idx =
                if animation.animation_indeces.len() - 1 == animation.current_animation_idx {
                    0
                } else {
                    animation.current_animation_idx + 1
                };

            let index = animation
                .animation_indeces
                .get(animation.current_animation_idx)
                .unwrap()
                .clone();

            sprite.index = index;
        }
    }
}

fn animate_cat(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut CurrentAnimation,
        &AnimationMap,
        &mut Player,
    )>,
) {
    for (mut timer, mut sprite, mut animation, map, _) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            animation.current_animation_idx =
                if animation.animation_indeces.len() - 1 == animation.current_animation_idx {
                    if animation.is_loop {
                        0
                    } else {
                        if !just_pressed_wasd(&keys)
                            && animation.current_animation != AnimationActions::IdleStand
                            && animation.current_animation != AnimationActions::Idle
                        {
                            println!("Keys where not pressed. Setting to idle");
                            animation.current_animation =
                                if animation.current_animation == AnimationActions::Jump {
                                    AnimationActions::IdleStand
                                } else {
                                    AnimationActions::Idle
                                }
                        }

                        let indeces = map.0.get(&animation.current_animation).unwrap().clone();

                        animation.animation_indeces = indeces.indices;
                        animation.current_animation_idx = 0;
                        0
                    }
                } else {
                    animation.current_animation_idx + 1
                };

            let index = animation
                .animation_indeces
                .get(animation.current_animation_idx)
                .unwrap()
                .clone();

            sprite.index = index;
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // load sprite sheet
    let texture_handle = asset_server.load("../assets/Cat-Sheet.png");
    // create texture atlas
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 8, 51, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_info = CAT_MAP.get(&DEBUG_ANIMATION).unwrap().clone();
    let transform = Transform::from_scale(Vec3::splat(6.0));
    let rect = Rect::new(0., 0., 19., 17.5);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(*animation_info.indices.get(0).unwrap()),
            transform: transform.clone(),
            ..default()
        },
        AnimationMap(CAT_MAP.clone()),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        CurrentAnimation {
            current_animation: DEBUG_ANIMATION,
            current_animation_idx: 0,
            animation_indeces: animation_info.indices,
            is_loop: animation_info.is_loop,
        },
        Player,
        Velocity {
            value: Vec3::splat(0.),
            prev: Vec3::splat(0.),
        },
        Gravity,
        Collider::new(rect, Vec2::new(1., 8.)),
        DebugBoundingBox::new(rect, Vec2::new(1., 8.)),
    ));
}

fn setup_map(mut commands: Commands, assets_server: Res<AssetServer>) {
    let idxs = vec![-300., 0., 300.0];
    let rect = Rect::new(0., 0., 300., 90.);

    for idx in idxs {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(300.0, 100.)),
                    ..default()
                },
                texture: assets_server.load("../assets/ground2.png"),
                transform: Transform::from_xyz(idx, -200., 1.),
                ..default()
            },
            Floor, // should just make everything but the player children of the World component
                   // and then just move the world when position
            Collider::new(rect, Vec2::new(0., 5.)),
            DebugBoundingBox::new(rect, Vec2::new(0., 5.)),
        ));
    }
}



fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_obstacle(mut commands: Commands) {
    let rect =Rect::new(0., 0., 50., 50.);
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(300., -70., 0.),
                ..default()
            },
            Floor,
            Collider::new(rect, Vec2::new(0., 5.)),
            DebugBoundingBox::new(rect, Vec2::new(0., 0.))
        ));

    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..default()
                },
                transform: Transform::from_xyz(-200., -70., 0.),
                ..default()
            },
            Floor,
            Collider::new(rect, Vec2::new(0., 5.)),
            DebugBoundingBox::new(rect, Vec2::new(0., 0.))
        ));
}

fn main() {
    App::new()
        // default_nearest to prevent blury sprites
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1000., 1000.).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_resource(World {
            pos: Vec2::new(0., 0.),
        })
        .add_systems(Startup, (camera_setup, setup_map, setup, setup_obstacle))
        .add_systems(PostStartup, draw_bounding_boxes)
        .add_systems(PreUpdate, collision::floor_collision)
        .add_systems(
            Update,
            (
                controlls::update_floor,
                controlls::controlls,
                controlls::update_player_vertical,
                animate_cat,
                gravity::gravity_system,
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

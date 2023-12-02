use std::collections::HashMap;

use bevy::prelude::*;
use sprite_animation_keys::{AnimationActions, AnimationInfo};

use crate::sprite_animation_keys::CAT_MAP;

// modules
pub mod controlls;
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
    is_loop: bool
}

#[derive(Component)]
pub struct Player;

fn _animation_test(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut CurrentAnimation,
        &AnimationMap,
        &mut TextureAtlasSprite,
    )>,
) {
    if keys.just_pressed(KeyCode::A) {
        for (mut current_animation, map, _) in &mut query {
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
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut CurrentAnimation,
        &AnimationMap
    )>,
) {
    for (mut timer, mut sprite, mut animation, map) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            animation.current_animation_idx =
                if animation.animation_indeces.len() - 1 == animation.current_animation_idx {
                    if animation.is_loop {
                        println!("Is Loop"); 
                        0
                    } else {
                        // update animation
                        animation.current_animation = AnimationActions::Idle;
                        let indeces = map
                            .0
                            .get(&animation.current_animation)
                            .unwrap()
                            .clone();

                        animation.animation_indeces = indeces.indices;
                        animation.current_animation_idx = 0;
                        // update animation

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

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(*animation_info.indices.get(0).unwrap()),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimationMap(CAT_MAP.clone()),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        CurrentAnimation {
            current_animation: DEBUG_ANIMATION,
            current_animation_idx: 0,
            animation_indeces: animation_info.indices,
            is_loop: animation_info.is_loop
        },
        Player,
    ));
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        // default_nearest to prevent blury sprites
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (camera_setup, setup))
        .add_systems(
            Update,
            (
                controlls::controlls,
                animate_cat,
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

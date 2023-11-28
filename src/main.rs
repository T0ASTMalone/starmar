use std::collections::HashMap;

use bevy::prelude::*;

use crate::sprite_animation_keys::CAT_MAP;

// modules
pub mod sprite_animation_keys;

const DEBUG_ANIMATION: usize = 0;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationMap(HashMap<usize, Vec<usize>>);

#[derive(Component)]
struct CurrentAnimation {
    current_animation: usize,
    current_animation_idx: usize,
    animation_indeces: Vec<usize>,
}

fn animation_test(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut CurrentAnimation,
        &AnimationMap,
        &mut TextureAtlasSprite,
    )>,
) {
    if keys.just_pressed(KeyCode::A) {
        for (mut current_animation, map, _) in &mut query {
            current_animation.current_animation =
                if current_animation.current_animation >= map.0.len() - 1 {
                    0
                } else {
                    current_animation.current_animation + 1
                };

            // TODO: look for better way than cloning
            let indeces = map
                .0
                .get(&current_animation.current_animation)
                .unwrap()
                .clone();

            current_animation.animation_indeces = indeces;
            current_animation.current_animation_idx = 0;
        }
    } else if keys.just_pressed(KeyCode::S) {
        // for debuging animations manually advances the current animation
        println!("Advancing Sprite Animation");
        for (mut animation, _, mut sprite) in &mut query {
            animation.current_animation_idx =
                if animation.animation_indeces.len() - 1 == animation.current_animation_idx {
                    0
                } else {
                    animation.current_animation_idx + 1
                };
            if animation.current_animation == DEBUG_ANIMATION {
                println!(
                    "Animation Indeces Index: {}",
                    animation.current_animation_idx
                );
            }

            let index = animation
                .animation_indeces
                .get(animation.current_animation_idx)
                .unwrap()
                .clone();

            if animation.current_animation == DEBUG_ANIMATION {
                println!("Actuall Sprite Index: {}", index);
            }

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
    )>,
) {
    for (mut timer, mut sprite, mut animation) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            animation.current_animation_idx =
                if animation.animation_indeces.len() - 1 == animation.current_animation_idx {
                    0
                } else {
                    animation.current_animation_idx + 1
                };
            if animation.current_animation == DEBUG_ANIMATION {
                println!(
                    "Animation Indeces Index: {}",
                    animation.current_animation_idx
                );
            }

            let index = animation
                .animation_indeces
                .get(animation.current_animation_idx)
                .unwrap()
                .clone();

            if animation.current_animation == DEBUG_ANIMATION {
                println!("Actuall Sprite Index: {}", index);
            }

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
    let initial_indices = CAT_MAP.get(&DEBUG_ANIMATION).unwrap().clone();

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(initial_indices.get(0).unwrap().clone()),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        AnimationMap(CAT_MAP.clone()),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        CurrentAnimation {
            current_animation: DEBUG_ANIMATION,
            current_animation_idx: 0,
            animation_indeces: initial_indices,
        },
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
            (animation_test, animate_cat, bevy::window::close_on_esc),
        )
        .run();
}

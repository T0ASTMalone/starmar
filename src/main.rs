use std::collections::HashMap;

use bevy::prelude::*;
use lazy_static::lazy_static;

const DEBUG_ANIMATION: usize = 21;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct CurrentAnimation {
    current_animation: usize,
    current_animation_idx: usize,
    animation_indeces: Vec<usize>,
}

// TODO: move to seperate file
lazy_static! {
    static ref CAT_MAP: HashMap<usize, Vec<usize>> = vec![
        (0, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]),
        (
            1,
            vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31]
        ),
        (
            2,
            vec![32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47]
        ),
        (3, vec![48, 49, 50, 51]),
        (4, vec![56, 57]),
        (5, vec![64, 65, 66]),
        (6, vec![72, 73, 74, 75]),
        (
            7,
            vec![80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95]
        ),
        (8, vec![96, 97, 98, 99, 104, 105, 106, 107]),
        (9, vec![112, 113, 114, 115, 120, 121, 122, 123]),
        (10, vec![128, 129, 130, 131]), // shifted back 1
        (
            11,
            vec![137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152]
        ),
        (12, vec![152, 153, 154, 155, 160, 161, 162, 163]),
        (
            13,
            vec![
                168, 169, 170, 171, 176, 177, 178, 179, 184, 185, 186, 187, 192, 193, 194, 195,
                200, 201, 202, 203, 208, 209, 210, 211, 216, 217, 218, 219, 224, 225, 226, 227,
                232, 233, 234, 235, 240, 241, 242, 243, 248, 249, 250, 251, 256, 257, 258, 259,
            ]
        ),
        (14, vec![264, 265]),
        (15, vec![272, 273, 274, 275, 276, 277, 278, 279]),
        (16, vec![280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291]),
        (17, vec![296, 297, 298, 299, 300, 301, 302, 303]),
        (18, vec![304, 305, 306, 307, 312, 313, 314, 315]),
        (19, vec![320, 321, 322, 323, 328, 329, 330, 331]),
        (20, vec![336, 337, 338, 339]),
        (21, vec![344, 345, 346, 347, 352, 353, 354, 355, 360, 361, 362, 363]) // poop

    ]
    .into_iter()
    .collect();
}

fn animation_test(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut CurrentAnimation, &mut TextureAtlasSprite)>,
) {
    if keys.just_pressed(KeyCode::A) {
        for (mut current_animation, _) in &mut query {
            current_animation.current_animation =
                if current_animation.current_animation >= CAT_MAP.len() {
                    0
                } else {
                    current_animation.current_animation + 1
                };

            println!("Current Animation: {}", current_animation.current_animation);
            // TODO: look for better way than cloning
            let indeces = CAT_MAP
                .get(&current_animation.current_animation)
                .unwrap()
                .clone();

            current_animation.animation_indeces = indeces;
            current_animation.current_animation_idx = 0;
        }
    } else if keys.just_pressed(KeyCode::S) {
        // for debuging animations manually advances the current animation
        println!("Advancing Sprite Animation");
        for (mut animation, mut sprite) in &mut query {
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
    return;
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

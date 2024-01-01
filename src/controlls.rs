use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, Input},
    prelude::Transform,
    sprite::TextureAtlasSprite,
    window::Window,
};

use crate::{
    sprite_animation_keys::AnimationActions, AnimationMap, CurrentAnimation, Floor, Player
};

fn update_animation(
    current_animation: &mut CurrentAnimation,
    map: &AnimationMap,
    animation: AnimationActions,
) {
    current_animation.current_animation = animation;
    let animation_info = map
        .0
        .get(&current_animation.current_animation)
        .unwrap()
        .clone();

    current_animation.animation_indeces = animation_info.indices;
    current_animation.current_animation_idx = 0;
    current_animation.is_loop = animation_info.is_loop;
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

pub fn update_floor(
    keys: Res<Input<KeyCode>>,
    window: Query<&Window>,
    mut query: Query<(&Floor, &mut Transform)>,
) {
    let diff = if keys.pressed(KeyCode::ShiftLeft) {
        10.
    } else {
        5.
    };

    let mut direction = Direction::None;

    for (_, mut transform) in &mut query {
        if keys.pressed(KeyCode::A) {
            direction = Direction::Left;
            transform.translation.x = transform.translation.x + diff;
        }

        if keys.pressed(KeyCode::D) {
            direction = Direction::Right;
            transform.translation.x = transform.translation.x - diff;
        }
    }
    // need to cach this
    let half_width = window.get_single().unwrap().width() / 2.;

    match direction {
        Direction::Left => {
            for (_, mut transform) in &mut query {
                if transform.translation.x - 149.5 > half_width {
                    // lol is x pos from the center of the tile?
                    let new_x = (299. * -2.) + diff + 149.5;
                    // println!("New X: {}", new_x);
                    transform.translation.x = new_x;
                }
            }
        }
        Direction::Right => {
            for (_, mut transform) in &mut query {
                if transform.translation.x + 149.5 < -half_width {
                    let new_x = 299. - diff + 149.5;
                    // println!("New X: {}", new_x);
                    transform.translation.x = new_x;
                }
            }
        }
        _ => {}
    };
}

pub fn just_pressed_wasd(keys: &Res<Input<KeyCode>>) -> bool {
    keys.any_just_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
        || keys.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
}

pub fn released_movement_keys(keys: &Res<Input<KeyCode>>) -> bool {
    keys.any_just_released([KeyCode::A, KeyCode::D])
}

pub fn controlls(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut Player,
        &mut CurrentAnimation,
        &AnimationMap,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut player, mut current_animation, map, mut sprite) in &mut query {
        if keys.just_pressed(KeyCode::A) {
            // turn left
            sprite.flip_x = true;
        }

        if keys.just_pressed(KeyCode::D) {
            // turn right
            sprite.flip_x = false;
        }

        if player.is_airborne {
            continue;
        }

        if keys.just_pressed(KeyCode::Return) {
            player.is_airborne = true;
            update_animation(&mut current_animation, map, AnimationActions::Jump);
            continue;
        }

        if keys.just_pressed(KeyCode::W) {
            // if croutched sit, if sitting stand
            update_animation(&mut current_animation, map, AnimationActions::IdleStand);
        }

        if released_movement_keys(&keys) {
            update_animation(&mut current_animation, map, AnimationActions::IdleStand)
        }

        if keys.pressed(KeyCode::A) {
            if !sprite.flip_x {
                sprite.flip_x = true;
            }

            if !keys.pressed(KeyCode::ShiftLeft)
                && current_animation.current_animation != AnimationActions::Walk
            {
                update_animation(&mut current_animation, map, AnimationActions::Walk)
            }

            if keys.pressed(KeyCode::ShiftLeft)
                && current_animation.current_animation != AnimationActions::Run
            {
                update_animation(&mut current_animation, map, AnimationActions::Run)
            }
        }

        if keys.just_pressed(KeyCode::S) {
            // croutch
            // if standing sit, if sitting croutch
            update_animation(&mut current_animation, map, AnimationActions::Croutch);
        }

        if keys.pressed(KeyCode::D) {
            if sprite.flip_x {
                sprite.flip_x = false;
            }
            if !keys.pressed(KeyCode::ShiftLeft)
                && current_animation.current_animation != AnimationActions::Walk
            {
                update_animation(&mut current_animation, map, AnimationActions::Walk)
            }

            if keys.pressed(KeyCode::ShiftLeft)
                && current_animation.current_animation != AnimationActions::Run
            {
                update_animation(&mut current_animation, map, AnimationActions::Run)
            }
        }
    }
}

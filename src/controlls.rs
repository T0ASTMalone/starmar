use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, Input},
    prelude::Transform,
    sprite::TextureAtlasSprite,
    window::Window,
};

use crate::{
    sprite_animation_keys::AnimationActions, AnimationMap, CurrentAnimation, Player, Floor,
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
    mut player_query: Query<&mut Player>,
    mut query: Query<(&Floor, &mut Transform)>,
) {
    let velocity = if keys.pressed(KeyCode::ShiftLeft) {
        10.
    } else {
        5.
    };

    let direction = if keys.pressed(KeyCode::A) {
        Direction::Left
    } else if keys.pressed(KeyCode::D) {
        Direction::Right
    } else {
        Direction::None
    };

    /* update tile location */

    // need to cach this
    let half_width = window.get_single().unwrap().width() / 2.;
    let prev_vel = player_query.get_single().unwrap().velocity;

    match direction {
        Direction::Left => {
            for (_, mut transform) in &mut query {
                if transform.translation.x - 149.5 > half_width {
                    // should use prev diff
                    transform.translation.x = (299. * -2.) + (prev_vel - velocity).abs() + 149.5;
                }
            }
        }
        Direction::Right => {
            for (_, mut transform) in &mut query {
                if transform.translation.x + 149.5 < -half_width {
                    transform.translation.x = 299. - (prev_vel - velocity).abs() + 149.5;
                }
            }
        }
        _ => {}
    };

    /* update tile pos */
    for (_, mut transform) in &mut query {
        match direction {
           Direction::Left => transform.translation.x = transform.translation.x + velocity,
           Direction::Right => transform.translation.x = transform.translation.x - velocity,
           _ => {}
        }
    }

    for mut player in &mut player_query {
        player.velocity = velocity;
    }
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

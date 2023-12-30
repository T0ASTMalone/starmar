use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, Input},
    prelude::Transform,
    sprite::TextureAtlasSprite,
};

use crate::{
    sprite_animation_keys::AnimationActions, AnimationMap, CurrentAnimation, Floor, Player,
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

pub fn floor_controlls(keys: Res<Input<KeyCode>>, mut query: Query<(&Floor, &mut Transform)>) {
    if keys.just_pressed(KeyCode::Down) {
        for (_, mut transform) in &mut query {
            transform.translation.y -= 1.;
            println!("New y corrd: {}", transform.translation.y);
        }
    }

    if keys.just_pressed(KeyCode::Up) {
        for (_, mut transform) in &mut query {
            transform.translation.y += 1.;
            println!("New y corrd: {}", transform.translation.y);
        }
    }

    if keys.pressed(KeyCode::A) {
        for (_, mut transform) in &mut query {
            transform.translation.x += if keys.pressed(KeyCode::ShiftLeft) {
                10.
            } else {
                5.
            };
            println!("New y corrd: {}", transform.translation.x);
        }
    }
    if keys.pressed(KeyCode::D) {
        for (_, mut transform) in &mut query {
            transform.translation.x -= if keys.pressed(KeyCode::ShiftLeft) {
                10.
            } else {
                5.
            };
            println!("New y corrd: {}", transform.translation.x);
        }
    }
}

pub fn controlls(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(
        &Player,
        &mut CurrentAnimation,
        &AnimationMap,
        &mut TextureAtlasSprite,
    )>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (_, mut current_animation, map, _) in &mut query {
            update_animation(&mut current_animation, map, AnimationActions::Jump);
        }
    }

    if keys.just_pressed(KeyCode::W) {
        // if croutched sit, if sitting stand
        for (_, mut current_animation, map, _) in &mut query {
            update_animation(&mut current_animation, map, AnimationActions::IdleStand);
        }
    }

    if keys.just_pressed(KeyCode::A) {
        // turn left
        for (_, _, _, mut sprite) in &mut query {
            sprite.flip_x = true;
        }
    }

    if keys.just_released(KeyCode::W)
        || keys.just_released(KeyCode::S)
        || keys.just_released(KeyCode::A)
        || keys.just_released(KeyCode::D)
    {
        for (_, mut current_animation, map, _) in &mut query {
            update_animation(&mut current_animation, map, AnimationActions::IdleStand)
        }
    }

    if keys.pressed(KeyCode::A) {
        for (_, mut current_animation, map, mut sprite) in &mut query {

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
    }

    if keys.just_pressed(KeyCode::S) {
        // croutch
        for (_, mut current_animation, map, _) in &mut query {
            // if standing sit, if sitting croutch
            update_animation(&mut current_animation, map, AnimationActions::Croutch);
        }
    }

    if keys.just_pressed(KeyCode::D) {
        // turn right
        for (_, _, _, mut sprite) in &mut query {
            sprite.flip_x = false;
        }
    }

    if keys.pressed(KeyCode::D) {
        for (_, mut current_animation, map, mut sprite) in &mut query {
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

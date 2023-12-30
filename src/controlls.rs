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
// TODO: only render a few floor tiles
// TODO: add floor tiles if approching edges
pub fn floor_controlls(keys: Res<Input<KeyCode>>, mut query: Query<(&Floor, &mut Transform)>) {
    
    if keys.just_pressed(KeyCode::Down) {
        for (_, mut transform) in &mut query {
            transform.translation.y -= 1.;
        }
    }

    if keys.just_pressed(KeyCode::Up) {
        for (_, mut transform) in &mut query {
            transform.translation.y += 1.;
        }
    }

    if keys.pressed(KeyCode::A) {
        for (_, mut transform) in &mut query {
            transform.translation.x += if keys.pressed(KeyCode::ShiftLeft) {
                10.
            } else {
                5.
            };
        }
    }
    if keys.pressed(KeyCode::D) {
        for (_, mut transform) in &mut query {
            transform.translation.x -= if keys.pressed(KeyCode::ShiftLeft) {
                10.
            } else {
                5.
            };
        }
    }
}

pub fn just_pressed_wasd(keys: &Res<Input<KeyCode>>) -> bool {
    keys.any_just_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D]) || keys.any_pressed([KeyCode::W, KeyCode::A, KeyCode::S, KeyCode::D])
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
    // TODO: add is jumping to Player. If jumping wait until animation is done before processing
    // keys
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
            continue
        }

        if keys.just_pressed(KeyCode::Space) {
            player.is_airborne = true;
            update_animation(&mut current_animation, map, AnimationActions::Jump);
            continue;
        } 

        if keys.just_pressed(KeyCode::W) {
            // if croutched sit, if sitting stand
            update_animation(&mut current_animation, map, AnimationActions::IdleStand);
        }

        if keys.just_released(KeyCode::A) || keys.just_released(KeyCode::D) {
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

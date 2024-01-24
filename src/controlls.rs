use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, Input},
    prelude::{MouseButton, Transform, Vec3},
    sprite::TextureAtlasSprite,
    window::Window,
};

use crate::{
    sprite_animation_keys::AnimationActions, AnimationMap, CurrentAnimation, Floor, Player,
    Velocity, collision::Collider,
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

fn is_moving_right(velocity: Vec3) -> bool {
    velocity.x > 0.
}

fn is_moving_left(velocity: Vec3) -> bool {
    velocity.x < 0.
}

pub fn update_floor(
    window: Query<&Window>,
    // time: Res<Time>,
    player_query: Query<(&Player, &Collider, &Velocity)>,
    mut query: Query<(&Floor, &mut Transform)>,
) {
    let (_, collider, velocity) = player_query.get_single().unwrap();

    // need to cach this
    let half_width = window.get_single().unwrap().width() / 2.;

    for (_, mut transform) in &mut query {
        
        if is_moving_left(velocity.value) {
            if collider.is_colliding.left {
                return;
            }
        }

        if is_moving_right(velocity.value) {
            if collider.is_colliding.right {
                return;
            }
        }
        
        /* move tile to front or back*/
        if is_moving_left(velocity.value) && (transform.translation.x - 150.) >= half_width {
            transform.translation.x = (300. * -2.) + 150.;
        }
        if is_moving_right(velocity.value) && (transform.translation.x + 150.) <= -half_width {
            transform.translation.x = 300. + 150.;
        }
        /* end move tile to front or back*/

        /* update tile pos */
        transform.translation.x -= velocity.value.x;
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
    mouse: Res<Input<MouseButton>>,
    mut query: Query<(
        &mut Player,
        &mut CurrentAnimation,
        &AnimationMap,
        &mut TextureAtlasSprite,
        &mut Velocity,
    )>,
) {
    for (mut player, mut current_animation, map, mut sprite, mut vel) in &mut query {
        vel.prev.x = vel.value.x;

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

        if mouse.just_pressed(MouseButton::Left) {
            update_animation(&mut current_animation, map, AnimationActions::AttackForward);
        }

        if keys.just_pressed(KeyCode::W) {
            // if croutched sit, if sitting stand
            update_animation(&mut current_animation, map, AnimationActions::IdleStand);
        }

        if released_movement_keys(&keys) {
            vel.value.x = 0.;
            update_animation(&mut current_animation, map, AnimationActions::IdleStand)
        }

        if keys.pressed(KeyCode::A) {
            if !sprite.flip_x {
                sprite.flip_x = true;
            }

            if !keys.pressed(KeyCode::ShiftLeft)
                && current_animation.current_animation != AnimationActions::Walk
            {
                vel.value.x = -10.;
                update_animation(&mut current_animation, map, AnimationActions::Walk)
            }

            if keys.pressed(KeyCode::ShiftLeft)
                && current_animation.current_animation != AnimationActions::Run
            {
                vel.value.x = -20.;
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
                vel.value.x = 10.;

                update_animation(&mut current_animation, map, AnimationActions::Walk)
            }

            if keys.pressed(KeyCode::ShiftLeft)
                && current_animation.current_animation != AnimationActions::Run
            {
                vel.value.x = 20.;
                update_animation(&mut current_animation, map, AnimationActions::Run)
            }
        }
    }
}

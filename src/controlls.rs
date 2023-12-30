use std::f32::{MAX, MIN};

use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, Input},
    prelude::{Transform, ViewVisibility, ResMut},
    sprite::TextureAtlasSprite,
};

use crate::{
    sprite_animation_keys::AnimationActions, AnimationMap, CurrentAnimation, Floor, Player, World,
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
    None
}

// TODO: only render a few floor tiles
// TODO: add floor tiles if approching edges for infinite game
// TODO: for finite game add map generation or map loading system
pub fn update_floor(
    keys: Res<Input<KeyCode>>,
    mut world: ResMut<crate::World>,
    mut query: Query<(&Floor, &mut Transform)>,
) {
    let diff = if keys.pressed(KeyCode::ShiftLeft) {
            10.
        } else {
            5.
        };

    let mut direction = Direction::None;
    let mut min: f32 = MAX;
    let mut max: f32 = MIN;

    for (_, mut transform) in &mut query {
        if transform.translation.x < min {
            min = transform.translation.x;
        }

        if transform.translation.x > max {
            max = transform.translation.x;
        }
        
        if keys.pressed(KeyCode::A) {
            direction = Direction::Left;
            transform.translation.x = transform.translation.x + diff;
        }

        if keys.pressed(KeyCode::D) {
            direction = Direction::Right;
            transform.translation.x = transform.translation.x - diff;
        }
    }

    let min_min = min + (299. * 2.);
    let max_max = max - (299. * 2.);


    world.pos.x = match direction {
         Direction::Left => {
            println!("Current min {}", min_min);
            for (_, mut transform) in &mut query {
                if transform.translation.x == (max + diff) && world.pos.x < min_min {
                    println!("New min : {}", min - 299. + diff + (299. * 2.));
                    transform.translation.x = min - 299. + diff;
                }
            }
            world.pos.x - diff
        },
        Direction::Right => {
            for (_, mut transform) in &mut query {
                if transform.translation.x == (min - diff) && world.pos.x > max_max {
                    transform.translation.x = max + 299. - diff;
                }
            }
            world.pos.x + diff
        },
        _ => world.pos.x
    };
    /*
    if Direction::Left {
        println!("moving first tile to back {} {}", min, max);

        for (_, mut transform, _) in &mut query {
            if transform.translation.x == min {
                transform.translation.x = max + 299.;
            }
        }
        //  move first to last pos + 299
    } else if keys.pressed(KeyCode::A) {
        // if key pressed A
        println!("moving last tile to front {} {}", min, max);
        // move last tile to first pos - 29
        for (_, mut transform, _) in &mut query {
            if transform.translation.x == max {
                transform.translation.x = min - 299.;
            }
        }
    }
    */
    // if visibility count > x
    /*if hid_count >= 3 {

        let mut min = MAX;
        let mut max = MIN;

        for (_, transform, _) in &mut query {
            if transform.translation.x < min {
                min = transform.translation.x;
            }

            if transform.translation.x > max {
                max = transform.translation.x;
            }
        } 

        //  if key pressed D
        if keys.pressed(KeyCode::D) {
            println!("moving first tile to back {} {}", min, max);

            for (_, mut transform, _) in &mut query {
                if transform.translation.x == min {
                    transform.translation.x = max + 299.;
                }
            }
            //  move first to last pos + 299
        } else if keys.pressed(KeyCode::A) {
            // if key pressed A
            println!("moving last tile to front {} {}", min, max);
            // move last tile to first pos - 29
            for (_, mut transform, _) in &mut query {
                if transform.translation.x == max {
                    transform.translation.x = min - 299.;
                }
            }
        }
    }
    */
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
            continue;
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

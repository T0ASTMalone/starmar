use bevy::{
    ecs::system::{Query, Res},
    input::{keyboard::KeyCode, Input},
    sprite::TextureAtlasSprite,
};

use crate::{sprite_animation_keys::AnimationActions, AnimationMap, CurrentAnimation, Player};

fn update_animation(current_animation: &mut CurrentAnimation, map: &AnimationMap, animation: AnimationActions) {
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
        println!("Pressed A");
        for (_, _, _, mut sprite) in &mut query {
            sprite.flip_x = true;
        }
    }

    if keys.just_pressed(KeyCode::S) {
        // croutch
        println!("Pressed S");
        for (_, mut current_animation, map, _) in &mut query {
            // if standing sit, if sitting croutch
            update_animation(&mut current_animation, map, AnimationActions::Croutch);
        }
    }

    if keys.just_pressed(KeyCode::D) {
        // turn right
        println!("Pressed D");
        for (_, _, _, mut sprite) in &mut query {
            sprite.flip_x = false;
        }
    }
}

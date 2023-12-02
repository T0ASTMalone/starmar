use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum AnimationActions {
    Idle, // Should these be just idle and we can add a duration
    IdleStand,
    IdleCroutch,
    IdleLong,

    Walk,
    Jog,
    Run,
    Sprint,
    Croutch,
    Sneak,
    Jump,
    Scared,

    TakeDamage,
    Die,
    Die2,
    Respawn,

    AttackForward,
    AttackBack,
    AttackRadial,

    // cat specific
    Hairball,
    Poop,
    Play,
    Nom,
    Hack,
    Stretch,
}

#[derive(Clone)]
pub struct AnimationInfo {
    pub indices: Vec<usize>,
    pub is_loop: bool,
}

impl AnimationInfo {
    fn new(indices: Vec<usize>, is_loop: bool) -> Self {
        AnimationInfo { indices, is_loop }
    }
}

lazy_static! {
    pub static ref CAT_MAP: HashMap<AnimationActions, AnimationInfo> = vec![
        (AnimationActions::Idle, AnimationInfo::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], true)), // idle
        (
            AnimationActions::IdleStand,
            AnimationInfo::new(vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31], true), // idle stand
        ),
        (
            AnimationActions::IdleLong,
            AnimationInfo::new(
            vec![
                168, 169, 170, 171, 176, 177, 178, 179, 184, 185, 186, 187, 192, 193, 194, 195,
                200, 201, 202, 203, 208, 209, 210, 211, 216, 217, 218, 219, 224, 225, 226, 227, // long idle / groom
                232, 233, 234, 235, 240, 241, 242, 243, 248, 249, 250, 251, 256, 257, 258, 259,
            ],
            true)
        ),
        (
            AnimationActions::IdleCroutch,
            AnimationInfo::new(vec![80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95], true) // sleep
        ),


        (
            AnimationActions::Walk,
            AnimationInfo::new(vec![32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47], true) // walk
        ),
        (AnimationActions::Jog, AnimationInfo::new(vec![48, 49, 50, 51], true)), // jog
        (AnimationActions::Run, AnimationInfo::new(vec![56, 57], true)), // run
        (AnimationActions::Sprint, AnimationInfo::new(vec![64, 65, 66], true)), // sprint
        (AnimationActions::Croutch, AnimationInfo::new(vec![72, 73, 74, 75], true)), // ley
        (AnimationActions::Sneak, AnimationInfo::new(vec![96, 97, 98, 99, 104, 105, 106, 107], true)), // crawl


        (AnimationActions::Jump, AnimationInfo::new(vec![152, 153, 154, 155, 160, 161, 162, 163], false)), // leap
        (
            AnimationActions::Scared,
            AnimationInfo::new(vec![137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152], true) // spooked
        ),


        (AnimationActions::TakeDamage, AnimationInfo::new(vec![264, 265], true)), // take damage
        (AnimationActions::Die, AnimationInfo::new(vec![272, 273, 274, 275, 276, 277, 278, 279], true)), // death 1
        (AnimationActions::Die2, AnimationInfo::new(vec![280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291], true)), // death 2 / spurt
        (AnimationActions::Respawn, AnimationInfo::new(vec![296, 297, 298, 299, 300, 301, 302, 303], true)), // respawn


        (AnimationActions::AttackForward, AnimationInfo::new(vec![304, 305, 306, 307, 312, 313, 314, 315], true)), // front attack
        (AnimationActions::AttackBack, AnimationInfo::new(vec![320, 321, 322, 323, 328, 329, 330, 331], true)), // back attack
        (AnimationActions::AttackRadial, AnimationInfo::new(vec![336, 337, 338, 339], true)), // front/back attack



        (AnimationActions::Hairball, AnimationInfo::new(vec![128, 129, 130, 131], true)), // hairball
        (AnimationActions::Poop, AnimationInfo::new(vec![344, 345, 346, 347, 352, 353, 354, 355, 360, 361, 362, 363], true)), // poop
        (
            AnimationActions::Play,
            AnimationInfo::new(
                vec![
                    368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379,
                    380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, // play
                ], 
                true
            )
        ),
        (AnimationActions::Nom, AnimationInfo::new(vec![392, 393], true)), // packman
        (AnimationActions::Hack, AnimationInfo::new(vec![400, 401], true)), // typing cat
        (AnimationActions::Stretch, AnimationInfo::new(vec![112, 113, 114, 115, 120, 121, 122, 123], true)), // stretch
    ]
    .into_iter()
    .collect();
}

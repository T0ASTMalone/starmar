use std::collections::HashMap;

use lazy_static::lazy_static;


enum Actions {
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
    PacMan,
    Type,
}

lazy_static! {
    pub static ref CAT_MAP: HashMap<usize, Vec<usize>> = vec![
        (0, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), // idle
        (
            1,
            vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 31] // idle stand
        ),
        (
            2,
            vec![32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47] // walk
        ),
        (3, vec![48, 49, 50, 51]), // jog
        (4, vec![56, 57]), // run
        (5, vec![64, 65, 66]), // sprint
        (6, vec![72, 73, 74, 75]), // ley
        (
            7,
            vec![80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95] // sleep
        ),
        (8, vec![96, 97, 98, 99, 104, 105, 106, 107]), // crawl
        (9, vec![112, 113, 114, 115, 120, 121, 122, 123]), // stretch
        (10, vec![128, 129, 130, 131]), // hairball
        (
            11,
            vec![137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152] // spooked
        ),
        (12, vec![152, 153, 154, 155, 160, 161, 162, 163]), // leap
        (
            13,
            vec![
                168, 169, 170, 171, 176, 177, 178, 179, 184, 185, 186, 187, 192, 193, 194, 195,
                200, 201, 202, 203, 208, 209, 210, 211, 216, 217, 218, 219, 224, 225, 226, 227, // long idle / groom
                232, 233, 234, 235, 240, 241, 242, 243, 248, 249, 250, 251, 256, 257, 258, 259,
            ]
        ),
        (14, vec![264, 265]), // take damage
        (15, vec![272, 273, 274, 275, 276, 277, 278, 279]), // death 1
        (16, vec![280, 281, 282, 283, 284, 285, 286, 287, 288, 289, 290, 291]), // death 2 / spurt
        (17, vec![296, 297, 298, 299, 300, 301, 302, 303]), // respawn
        (18, vec![304, 305, 306, 307, 312, 313, 314, 315]), // front attack
        (19, vec![320, 321, 322, 323, 328, 329, 330, 331]), // back attack
        (20, vec![336, 337, 338, 339]), // front/back attack
        (21, vec![344, 345, 346, 347, 352, 353, 354, 355, 360, 361, 362, 363]), // poop
        (22, vec![
            368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378, 379,
            380, 381, 382, 383, 384, 385, 386, 387, 388, 389, 390, 391, // play
        ]),
        (23, vec![392, 393]), // packman
        (24, vec![400, 401]), // typing cat
    ]
    .into_iter()
    .collect();
}

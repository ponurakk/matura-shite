use imageproc::point::Point;

pub const SHAPE_1: [Point<i32>; 11] = [
    Point { x: 263, y: 69 },
    Point { x: 215, y: 67 },
    Point { x: 181, y: 56 },
    Point { x: 191, y: 47 },
    Point { x: 217, y: 42 },
    Point { x: 245, y: 5 },
    Point { x: 263, y: 24 },
    Point { x: 277, y: 3 },
    Point { x: 308, y: 43 },
    Point { x: 342, y: 52 },
    Point { x: 318, y: 66 },
];

pub const SHAPE_2: [Point<i32>; 7] = [
    Point { x: 220, y: 71 },
    Point { x: 301, y: 71 },
    Point { x: 301, y: 94 },
    Point { x: 288, y: 112 },
    Point { x: 262, y: 123 },
    Point { x: 231, y: 112 },
    Point { x: 220, y: 94 },
];

pub const SHAPE_3: [Point<i32>; 15] = [
    Point { x: 244, y: 123 },
    Point { x: 260, y: 126 },
    Point { x: 272, y: 123 },
    Point { x: 273, y: 139 },
    Point { x: 338, y: 148 },
    Point { x: 404, y: 345 },
    Point { x: 379, y: 354 },
    Point { x: 317, y: 186 },
    Point { x: 319, y: 327 },
    Point { x: 220, y: 327 },
    Point { x: 214, y: 186 },
    Point { x: 147, y: 356 },
    Point { x: 125, y: 346 },
    Point { x: 185, y: 154 },
    Point { x: 243, y: 139 },
];

pub const SHAPE_4: [Point<i32>; 11] = [
    Point { x: 221, y: 330 },
    Point { x: 321, y: 330 },
    Point { x: 351, y: 542 },
    Point { x: 406, y: 541 },
    Point { x: 405, y: 563 },
    Point { x: 313, y: 577 },
    Point { x: 270, y: 378 },
    Point { x: 235, y: 574 },
    Point { x: 149, y: 567 },
    Point { x: 143, y: 545 },
    Point { x: 193, y: 545 },
];

pub const SHAPE_5: [Point<i32>; 4] = [
    Point { x: 125, y: 350 },
    Point { x: 146, y: 358 },
    Point { x: 78, y: 513 },
    Point { x: 58, y: 501 },
];

pub const SHAPE_6: [Point<i32>; 4] = [
    Point { x: 22, y: 480 },
    Point { x: 78, y: 517 },
    Point { x: 68, y: 539 },
    Point { x: 3, y: 523 },
];

pub const SHAPES: [&[Point<i32>]; 6] = [&SHAPE_1, &SHAPE_2, &SHAPE_3, &SHAPE_4, &SHAPE_5, &SHAPE_6];

#![allow(missing_docs)]
#![allow(clippy::unwrap_used)]

mod shapes;
use std::{
    fs::create_dir_all,
    io::Cursor,
    sync::atomic::{AtomicUsize, Ordering},
};

use rayon::prelude::*;
use shapes::SHAPES;

use image::{ImageReader, Rgb, RgbImage};
use imageproc::drawing::draw_polygon_mut;

static GLOBAL_COUNTER: AtomicUsize = AtomicUsize::new(0);

const COLORS: [Rgb<u8>; 7] = [
    Rgb([255, 0, 0]),
    Rgb([0, 255, 0]),
    Rgb([0, 0, 255]),
    Rgb([255, 255, 0]),
    Rgb([255, 0, 255]),
    Rgb([0, 255, 255]),
    Rgb([0, 0, 0]),
];

// adjacency list (0-based indexing)
const ADJ: [&[usize]; 6] = [
    &[1],       // 1
    &[0, 2],    // 2
    &[1, 3, 4], // 3
    &[2],       // 4
    &[2, 5],    // 5
    &[4],       // 6
];

fn is_valid(node: usize, colors: &[Option<usize>; 6], color: usize) -> bool {
    for &neighbor in ADJ[node] {
        if colors[neighbor] == Some(color) {
            return false;
        }
    }
    true
}

// sequential recursion for a partial assignment
fn generate_seq(
    node: usize,
    colors: &mut [Option<usize>; 6],
    base_img: &RgbImage,
) -> Result<(), Box<dyn std::error::Error>> {
    if node == 6 {
        let mut img = base_img.clone();

        for i in 0..6 {
            let color_index = colors[i].unwrap();
            draw_polygon_mut(&mut img, SHAPES[i], COLORS[color_index]);
        }

        let id = GLOBAL_COUNTER.fetch_add(1, Ordering::SeqCst);
        if GLOBAL_COUNTER.load(Ordering::Relaxed).is_multiple_of(1000) {
            println!(
                "Generated approx {} solutions",
                GLOBAL_COUNTER.load(Ordering::Relaxed)
            );
        }
        img.save(format!("out/output_{id:05}.jpg"))?;

        return Ok(());
    }

    for color in 0..7 {
        if is_valid(node, colors, color) {
            colors[node] = Some(color);
            generate_seq(node + 1, colors, base_img)?;
            colors[node] = None;
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    create_dir_all("./out")?;
    let img_bytes: &[u8] = include_bytes!("../ludzik-kurwa.jpg");

    let cursor = Cursor::new(img_bytes);

    let base_img: RgbImage = ImageReader::new(cursor)
        .with_guessed_format()?
        .decode()?
        .to_rgb8();

    (0..7).into_par_iter().for_each(|color0| {
        let mut colors: [Option<usize>; 6] = [None; 6];
        colors[0] = Some(color0);

        generate_seq(1, &mut colors, &base_img).unwrap();
    });

    println!("All solutions generated.");

    Ok(())
}

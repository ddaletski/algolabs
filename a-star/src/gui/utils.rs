use crate::traits::Solver;
use image::{GrayImage, RgbaImage};

pub fn maze_image(solver: &impl Solver) -> RgbaImage {
    let solver_grid = solver.inspect();
    let size = solver.maze().size;
    let gray_img = GrayImage::from_raw(size.width, size.height, solver_grid).unwrap();

    let mut palette: Vec<(u8, u8, u8)> = Vec::new();
    palette.resize(256, (0, 0, 0));
    palette[0] = (0, 0, 0); // free is black
    palette[1] = (255, 0, 0); // wall is red
    palette[2] = (0, 255, 0); // checked is green
    palette[3] = (255, 255, 0); // queued is yellow
    palette[4] = (0, 0, 255); // source is blue
    palette[5] = (255, 0, 255); // destination is purple

    let mut rgba_img: RgbaImage = gray_img.expand_palette(&palette[..], None);

    rgba_img = image::imageops::resize(
        &rgba_img,
        size.width * crate::CELL_SCALE,
        size.height * crate::CELL_SCALE,
        image::imageops::FilterType::Nearest,
    );

    rgba_img
}

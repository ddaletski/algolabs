use crate::common::Size;
use eframe::egui;
use image;

pub struct Animation {
    name: String,
    texture: egui::TextureHandle,
}

impl Animation {
    pub fn new(name: &str, ctx: &egui::Context, rgba_img: &image::RgbaImage) -> Self {
        let img = egui::ColorImage::from_rgba_unmultiplied(
            [rgba_img.height() as usize, rgba_img.width() as usize],
            &rgba_img.as_raw().as_slice(),
        );

        let texture = ctx.load_texture(name, img);

        Self {
            name: name.to_owned(),
            texture,
        }
    }

    pub fn update(&mut self, rgba_img: &image::RgbaImage) {
        let img = egui::ColorImage::from_rgba_unmultiplied(
            [rgba_img.height() as usize, rgba_img.width() as usize],
            &rgba_img.as_raw().as_slice(),
        );

        self.texture.set(img);
    }

    pub fn size(&self) -> Size {
        let size_raw = self.texture.size();
        Size {
            width: size_raw[1] as u32,
            height: size_raw[0] as u32,
        }
    }

    pub fn texture(&self) -> &egui::TextureHandle {
        &self.texture
    }
}

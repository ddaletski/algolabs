use crate::common::Size;
use eframe::egui;
use image;

pub struct Animation {
    texture: egui::TextureHandle,
}

impl Animation {
    pub fn new(name: &str, ctx: &egui::Context, rgba_img: &image::RgbaImage) -> Self {
        let img = egui::ColorImage::from_rgba_unmultiplied(
            [rgba_img.width() as usize, rgba_img.height() as usize],
            &rgba_img.as_raw().as_slice(),
        );

        let texture = ctx.load_texture(name, img);

        Self { texture }
    }

    pub fn update(&mut self, rgba_img: &image::RgbaImage) {
        assert!(rgba_img.width() == self.texture.size()[0] as u32);
        assert!(rgba_img.height() == self.texture.size()[1] as u32);
        // assert(self.texture().size() == rgba_img.get

        let img = egui::ColorImage::from_rgba_unmultiplied(
            [rgba_img.width() as usize, rgba_img.height() as usize],
            &rgba_img.as_raw().as_slice(),
        );

        self.texture.set(img);
    }

    pub fn size(&self) -> Size {
        let size_raw = self.texture.size();
        Size {
            width: size_raw[0] as u32,
            height: size_raw[1] as u32,
        }
    }

    pub fn texture(&self) -> &egui::TextureHandle {
        &self.texture
    }
}

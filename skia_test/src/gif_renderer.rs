use std::{borrow::BorrowMut, fs::File};

use gl::types::GLuint;
use image::{codecs::gif::GifDecoder, AnimationDecoder, Frame, RgbaImage};
use skia_safe::{Canvas, ColorSpace, ISize, Image, ImageInfo, Point};

pub struct GifTextureRenderer {
    frame_idx: usize,
    texture_buffer: Vec<GLuint>,
    frame_buffer: Vec<RgbaImage>,
}

impl GifTextureRenderer {
    pub fn load_gif(file: &str) -> anyhow::Result<Self> {
        let mut g = GifTextureRenderer {
            frame_idx: 0,
            texture_buffer: Vec::new(),
            frame_buffer: Vec::new(),
        };

        println!("Loading image {}", file);
        let mut f = File::open(file)?;
        println!("Decoding gif");
        let mut frames = GifDecoder::new(f)?.into_frames();

        println!("Loading frames into GPU");
        while let Some(Ok(frame)) = frames.next() {
            g.frame_buffer.push(frame.into_buffer());
            let frame = g.frame_buffer.last().unwrap();
            // let mut texture: GLuint = 0;
            // unsafe {
            //     gl::GenTextures(1, &mut texture);
            //     gl::BindTexture(gl::TEXTURE_2D, texture);
            //     gl::TexImage2D(
            //         gl::TEXTURE_2D,
            //         0,
            //         gl::RGBA as i32,
            //         frame.width() as i32,
            //         frame.height() as i32,
            //         0,
            //         gl::RGBA,
            //         gl::UNSIGNED_BYTE,
            //         frame.as_raw().as_ptr() as *const _,
            //     )
            // }
            // g.texture_buffer.push(texture);
            println!("Loaded frame...");
        }

        Ok(g)
    }

    pub fn render_frame(&mut self, frame: usize, canvas: &mut Canvas) -> usize {
        let mut frame = frame;
        // let mut recording_context = canvas.recording_context().unwrap();
        // let recording_context = recording_context.borrow_mut();

        if frame == 6 {
            self.frame_idx = if self.frame_idx + 1 >= self.frame_buffer.len() {
                0
            } else {
                self.frame_idx + 1
            };
            frame = 0
        } else {
            frame += 1
        }

        let this_frame = self.frame_buffer.get(self.frame_idx).unwrap();
        // let backend_texture = unsafe {
        //     skia_safe::gpu::BackendTexture::new_gl(
        //         (this_frame.width() as i32, this_frame.height() as i32),
        //         skia_safe::gpu::MipMapped::No,
        //         skia_safe::gpu::gl::TextureInfo::from_target_and_id(
        //             gl::TEXTURE_2D,
        //             *self.texture_buffer.get(self.frame_idx).unwrap(),
        //         ),
        //     )
        // };
        let img_info = ImageInfo::new(
            ISize::new(this_frame.width() as i32, this_frame.height() as i32),
            skia_safe::ColorType::RGBA8888,
            skia_safe::AlphaType::Opaque,
            ColorSpace::new_srgb(),
        );
        let image = Image::from_raster_data(
            &img_info,
            unsafe { skia_safe::Data::new_bytes(this_frame.as_raw()) },
            this_frame.width() as usize * skia_safe::ColorType::RGBA8888.bytes_per_pixel() 
        ).unwrap();
        canvas.draw_image(image, Point::new(0.0, 0.0), None);
        frame
    }
}

use eframe::glow::{self, Framebuffer, HasContext};
use image::ImageBuffer;
use std::sync::Arc;

type ImageBufferRgba = ImageBuffer<image::Rgba<u8>, Vec<u8>>;

pub trait RenderFramebuffer {
    fn gl(&self) -> &Arc<glow::Context>;
    fn before_paint(&self) {}
    fn after_paint(&self) {}
    fn get_image(&self) -> ImageBufferRgba;
}

pub struct BasicRenderer {
    gl: Arc<glow::Context>,
    width: i32,
    height: i32,
    framebuffer: Framebuffer,
}

impl BasicRenderer {
    pub fn new(gl: Arc<glow::Context>, width: i32, height: i32) -> Self {
        unsafe {
            let framebuffer = gl
                .create_framebuffer()
                .expect("Failed to create framebuffer");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));

            let texture = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(None),
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );

            let depthbuffer = gl
                .create_renderbuffer()
                .expect("Failed to create depthbuffer");
            gl.bind_renderbuffer(glow::RENDERBUFFER, Some(depthbuffer));
            gl.renderbuffer_storage(glow::RENDERBUFFER, glow::DEPTH_COMPONENT24, width, height);
            gl.framebuffer_renderbuffer(
                glow::FRAMEBUFFER,
                glow::DEPTH_ATTACHMENT,
                glow::RENDERBUFFER,
                Some(depthbuffer),
            );

            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture),
                0,
            );
            assert_eq!(
                gl.check_framebuffer_status(glow::FRAMEBUFFER),
                glow::FRAMEBUFFER_COMPLETE
            );

            gl.bind_framebuffer(glow::FRAMEBUFFER, None);

            Self {
                gl,
                width,
                height,
                framebuffer,
            }
        }
    }

    fn bind(&self) {
        unsafe {
            self.gl
                .bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
            self.gl.viewport(0, 0, self.width, self.height);
        }
    }

    fn clear(&self) {
        unsafe {
            self.gl.clear_color(0.0, 0.0, 0.0, 0.0);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }
}

impl RenderFramebuffer for BasicRenderer {
    fn gl(&self) -> &Arc<glow::Context> {
        &self.gl
    }

    fn before_paint(&self) {
        self.bind();
        self.clear();
    }

    fn get_image(&self) -> ImageBufferRgba {
        get_image(&self.gl, self.width, self.height, self.framebuffer)
    }
}

pub struct MultisampleRenderer {
    gl: Arc<glow::Context>,
    width: i32,
    height: i32,
    framebuffer_multisample: Framebuffer,
    framebuffer_resolve: Framebuffer,
}

impl MultisampleRenderer {
    pub fn new(gl: Arc<glow::Context>, width: i32, height: i32, samples: i32) -> Self {
        assert!(samples > 0);
        unsafe {
            let framebuffer_multisample = gl
                .create_framebuffer()
                .expect("Failed to create framebuffer");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer_multisample));

            let texture_multisample = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(glow::TEXTURE_2D_MULTISAMPLE, Some(texture_multisample));
            gl.tex_image_2d_multisample(
                glow::TEXTURE_2D_MULTISAMPLE,
                samples,
                glow::RGBA8 as i32,
                width,
                height,
                true,
            );
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D_MULTISAMPLE,
                Some(texture_multisample),
                0,
            );

            let depthbuffer_multisample = gl
                .create_renderbuffer()
                .expect("Failed to create depthbuffer");
            gl.bind_renderbuffer(glow::RENDERBUFFER, Some(depthbuffer_multisample));
            gl.renderbuffer_storage_multisample(
                glow::RENDERBUFFER,
                samples,
                glow::DEPTH_COMPONENT24,
                width,
                height,
            );
            gl.framebuffer_renderbuffer(
                glow::FRAMEBUFFER,
                glow::DEPTH_ATTACHMENT,
                glow::RENDERBUFFER,
                Some(depthbuffer_multisample),
            );

            assert_eq!(
                gl.check_framebuffer_status(glow::FRAMEBUFFER),
                glow::FRAMEBUFFER_COMPLETE
            );

            let framebuffer_resolve = gl
                .create_framebuffer()
                .expect("Failed to create framebuffer");
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer_resolve));

            let texture_resolve = gl.create_texture().expect("Failed to create texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(texture_resolve));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA8 as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(None),
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(texture_resolve),
                0,
            );

            assert_eq!(
                gl.check_framebuffer_status(glow::FRAMEBUFFER),
                glow::FRAMEBUFFER_COMPLETE
            );

            gl.bind_framebuffer(glow::FRAMEBUFFER, None);

            Self {
                gl,
                width,
                height,
                framebuffer_multisample,
                framebuffer_resolve,
            }
        }
    }

    fn bind(&self) {
        unsafe {
            self.gl
                .bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer_multisample));
            self.gl.viewport(0, 0, self.width, self.height);
        }
    }

    fn clear(&self) {
        unsafe {
            self.gl.clear_color(0.0, 0.0, 0.0, 0.0);
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
        }
    }

    fn resolve(&self) {
        unsafe {
            self.gl
                .bind_framebuffer(glow::READ_FRAMEBUFFER, Some(self.framebuffer_multisample));
            self.gl
                .bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(self.framebuffer_resolve));
            self.gl.blit_framebuffer(
                0,
                0,
                self.width,
                self.height,
                0,
                0,
                self.width,
                self.height,
                glow::COLOR_BUFFER_BIT,
                glow::NEAREST,
            );
        }
    }
}

impl RenderFramebuffer for MultisampleRenderer {
    fn gl(&self) -> &Arc<glow::Context> {
        &self.gl
    }

    fn before_paint(&self) {
        self.bind();
        self.clear();
    }

    fn after_paint(&self) {
        self.resolve();
    }

    fn get_image(&self) -> ImageBufferRgba {
        get_image(&self.gl, self.width, self.height, self.framebuffer_resolve)
    }
}

fn get_image(
    gl: &glow::Context,
    width: i32,
    height: i32,
    framebuffer: Framebuffer,
) -> ImageBufferRgba {
    let img_size = (width * height * 4) as usize;
    let mut pixels = vec![0u8; img_size];

    unsafe {
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
        gl.read_pixels(
            0,
            0,
            width,
            height,
            glow::RGBA,
            glow::UNSIGNED_BYTE,
            glow::PixelPackData::Slice(Some(&mut pixels)),
        );
        gl.bind_framebuffer(glow::FRAMEBUFFER, None);
    }

    // OpenGL の画像は上下反転しているので直す
    let mut flipped_pixels = Vec::with_capacity(img_size);
    for y in 0..height {
        for x in 0..width {
            let i = (((height - y - 1) * width + x) * 4) as usize;
            flipped_pixels.extend_from_slice(&pixels[i..i + 4]);
        }
    }

    let img: ImageBufferRgba = ImageBuffer::from_raw(width as u32, height as u32, flipped_pixels)
        .expect("Failed to crerate ImageBuffer");

    img
}

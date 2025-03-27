use super::{utils::replace_extension, BlockViewScene};
use crate::{
    gl_renderer::{MultisampleFramebuffer, OrbitCamera, SceneRenderer},
    sw_block_definition::{Definition, SwBlockDefinitionMeshes},
};
use eframe::glow::Context;
use std::{
    path::PathBuf,
    sync::{mpsc, Arc},
};

pub type RenderMessageTuple = (Arc<Definition>, Arc<SwBlockDefinitionMeshes>, String);

pub struct ImageRenderer {
    rx: mpsc::Receiver<RenderMessageTuple>,
    camera: OrbitCamera,
    scene: BlockViewScene,
    renderer: SceneRenderer,
    framebuffer: MultisampleFramebuffer,
    save_path: PathBuf,
    append_filename: bool,
}

impl ImageRenderer {
    pub fn new(
        rx: mpsc::Receiver<RenderMessageTuple>,
        gl: &Arc<Context>,
        camera: &OrbitCamera,
        framebuffer: MultisampleFramebuffer,
        save_path: PathBuf,
        append_filename: bool,
    ) -> Self {
        let scene = BlockViewScene::default();
        let renderer = SceneRenderer::new(gl, scene.scene());

        Self {
            rx,
            camera: camera.clone(),
            scene,
            renderer,
            framebuffer,
            save_path,
            append_filename,
        }
    }

    pub fn update(&mut self) {
        loop {
            if let Ok((data, meshes, filename)) = self.rx.try_recv() {
                self.scene.update(&Some(data), &Some(meshes));
                self.framebuffer.paint(&mut self.renderer, &self.camera);
                let image = self.framebuffer.get_image();
                let _result = if self.append_filename {
                    image.save(self.save_path.join(replace_extension(&filename, "png")))
                } else {
                    image.save(&self.save_path)
                };
            } else {
                return;
            }
        }
    }
}

#[derive(Debug)]
pub enum ProgressMessage {
    Progress(usize),
    Done,
}

#[derive(Debug)]
pub struct SaveImageProgress {
    rx: mpsc::Receiver<ProgressMessage>,
    current: usize,
    total: usize,
    done: bool,
    message: Option<String>,
}

impl SaveImageProgress {
    pub fn new(rx: mpsc::Receiver<ProgressMessage>, total: usize) -> Self {
        Self {
            rx,
            current: 0,
            total,
            done: false,
            message: None,
        }
    }

    pub fn update(&mut self) {
        if let Ok(mes) = self.rx.try_recv() {
            match mes {
                ProgressMessage::Progress(value) => {
                    self.current = value;
                }
                ProgressMessage::Done => {
                    self.done = true;
                }
            }
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn progress(&self) -> f32 {
        self.current as f32 / self.total as f32
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn message(&self) -> &Option<String> {
        &self.message
    }
}

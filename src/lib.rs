use std::path::PathBuf;

pub mod default_animations;
/// struct used to define renderable objects.
/// used to serialize retreived mesh data into a unified intermediate format, which can be used by
/// renderers to create graphics
pub mod render_data;
pub mod skeleton;

pub fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub fn animations() -> PathBuf {
    root().join("Animations")
}

pub fn skeleton() -> PathBuf {
    root().join("Skeleton")
}

pub fn textures() -> PathBuf {
    root().join("Textures")
}

pub fn mesh() -> PathBuf {
    root().join("Mesh")
}

pub fn shaders() -> PathBuf {
    root().join("Shaders")
}

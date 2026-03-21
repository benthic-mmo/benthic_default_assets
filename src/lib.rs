use std::path::PathBuf;

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

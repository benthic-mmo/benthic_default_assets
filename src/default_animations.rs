use crate::skeleton::JointName;
use glam::{Quat, Vec3};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone)]
pub struct JointAnimation {
    pub joint: JointName,
    pub translations: &'static [(f32, Vec3)],
    pub rotations: &'static [(f32, Quat)],
    pub scales: &'static [(f32, Vec3)],
}

#[derive(EnumString, Display, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefaultAnimationName {
    #[strum(serialize = "Stand")]
    Stand,

    #[strum(serialize = "Bow")]
    Bow,
}

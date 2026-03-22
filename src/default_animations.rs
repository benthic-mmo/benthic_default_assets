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

use std::path::PathBuf;
use uuid::Uuid;

macro_rules! define_animations {
    (
        $( $name:ident => $uuid:expr ),* $(,)?
    ) => {
        #[derive(Debug, Copy, Clone, Display, EnumString, PartialEq, Eq, Hash)]
        pub enum DefaultAnimation {
            $( $name ),*
        }

        impl DefaultAnimation {
            pub fn uuid(&self) -> Uuid {
                match self {
                    $(
                        DefaultAnimation::$name => Uuid::parse_str($uuid).unwrap(),
                    )*
                }
            }

            pub fn path(&self) -> PathBuf {
                PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("assets")
                    .join("animations")
                    .join(format!("{}.glb", self.to_string()))
            }

            pub fn from_uuid(uuid: &Uuid) -> Option<Self> {
                $(
                    if Uuid::parse_str($uuid).unwrap() == *uuid {
                        return Some(DefaultAnimation::$name);
                    }
                )*
                None
            }
        }
    };
}

define_animations! {
    Stand => "2408fe9e-df1d-1d7d-f4ff-1384fa7b350f",
    Bow => "82e99230-c906-1403-4d9c-3889dd98daba",
}

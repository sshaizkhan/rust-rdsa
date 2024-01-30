#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum JointType {
    Prismatic,
    Revolute,
    Fixed,
    Floating,
    Planar,
    Continuous,
}

impl JointType {
    pub fn as_str(&self) -> &'static str {
        match self {
            JointType::Prismatic => "prismatic",
            JointType::Revolute => "revolute",
            JointType::Fixed => "fixed",
            JointType::Floating => "floating",
            JointType::Planar => "planar",
            JointType::Continuous => "continuous",
        }
    }
}

#[derive(Debug, Clone)]
pub struct JointAttributes {
    pub joint_name: String,
    pub joint_type: JointType,
}

impl Default for JointAttributes {
    fn default() -> Self {
        Self {
            joint_name: String::new(),
            joint_type: JointType::Revolute,
        }
    }
}

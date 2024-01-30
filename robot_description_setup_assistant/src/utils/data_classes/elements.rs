#![allow(dead_code)]

#[derive(Debug)]
pub struct Origin {
    pub xyz: (f64, f64, f64),
    pub rpy: (f64, f64, f64),
}
impl Default for Origin {
    fn default() -> Self {
        Self {
            xyz: (0.0, 0.0, 0.0),
            rpy: (0.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug)]
pub struct Calibration {
    pub rising: Option<f64>,
    pub falling: f64,
}
impl Default for Calibration {
    fn default() -> Self {
        Self {
            rising: Some(0.0),
            falling: 0.0,
        }
    }
}

#[derive(Debug)]
pub struct Dynamics {
    pub damping: Option<f64>,
    pub friction: Option<f64>,
}
impl Default for Dynamics {
    fn default() -> Self {
        Self {
            damping: Some(0.0),
            friction: Some(0.0),
        }
    }
}

#[derive(Debug)]
pub struct Limits {
    pub lower: Option<f64>,
    pub upper: Option<f64>,
    pub effort: Option<f64>,
    pub velocity: Option<f64>,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            lower: Some(0.0),
            upper: Some(0.0),
            effort: Some(0.0),
            velocity: Some(0.0),
        }
    }
}

#[derive(Debug)]
pub struct JointMimic {
    pub joint: String,
    pub multiplier: Option<f64>,
    pub offset: Option<f64>,
}

impl Default for JointMimic {
    fn default() -> Self {
        Self {
            joint: String::new(),
            multiplier: None,
            offset: None,
        }
    }
}

#[derive(Debug, Default)]
pub struct SafetyController {
    pub soft_lower_limit: f64,
    pub soft_upper_limit: f64,
    pub k_position: f64,
    pub k_velocity: f64,
}

#[derive(Debug)]
pub struct SafetyParams {
    pub safety_pos_margin: f64,
    pub safety_k_position: f64,
}

impl Default for SafetyParams {
    fn default() -> Self {
        Self {
            safety_pos_margin: 0.0,
            safety_k_position: 0.0,
        }
    }
}

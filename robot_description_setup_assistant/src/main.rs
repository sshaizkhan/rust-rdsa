mod utils {
    pub mod data_classes {
        pub mod attributes;
        pub mod elements;
    }
    pub mod link;
    mod cylinder_inertia;
}

fn main() {
    // Since you're inside `main.rs`, you need to refer to `attributes` through the `utils::data_classes` path
    let joint_attrs = utils::data_classes::attributes::JointAttributes {
        joint_name: "elbow_joint".to_string(),
        joint_type: utils::data_classes::attributes::JointType::Revolute,
    };

    println!(
        "Joint name: {}, type: {}",
        joint_attrs.joint_name,
        joint_attrs.joint_type.as_str()
    );

    let safety_controller = utils::data_classes::elements::SafetyController {
        soft_lower_limit: 0.0,
        soft_upper_limit: 0.0,
        k_position: 0.0,
        k_velocity: 0.0,
    };

    println!(
        "Soft Lower limit: {:?}, Soft Upper limit: {:?}, K Position: {:?}, K Velocity: {:?}",
        safety_controller.soft_lower_limit,
        safety_controller.soft_upper_limit,
        safety_controller.k_position,
        safety_controller.k_velocity
    );

    let origin = utils::data_classes::elements::Origin {
        xyz: (0.0, 0.0, 0.0),
        rpy: (0.0, 0.0, 0.0),
    };

    println!("Origin: {:?}, {:?}", origin.xyz, origin.rpy);

    let calibration = utils::data_classes::elements::Calibration {
        rising: Some(0.4),
        falling: 0.0,
    };

    println!(
        "Rising: {:?}, Falling: {:?}",
        calibration.rising, calibration.falling
    );

    let dynamics = utils::data_classes::elements::Dynamics {
        damping: Some(0.0),
        friction: Some(0.0),
    };

    println!(
        "Damping: {:?}, Friction: {:?}",
        dynamics.damping, dynamics.friction
    );

    let limits = utils::data_classes::elements::Limits {
        lower: Some(0.0),
        upper: Some(0.0),
        effort: Some(0.0),
        velocity: Some(0.0),
    };

    println!(
        "Lower: {:?}, Upper: {:?}, Effort: {:?}, Velocity: {:?}",
        limits.lower, limits.upper, limits.effort, limits.velocity
    );

    let joint_mimic = utils::data_classes::elements::JointMimic {
        joint: "elbow_joint".to_string(),
        multiplier: Some(0.0),
        offset: Some(0.0),
    };

    println!(
        "Joint: {:?}, Multiplier: {:?}, Offset: {:?}",
        joint_mimic.joint, joint_mimic.multiplier, joint_mimic.offset
    );

    let safety_params = utils::data_classes::elements::SafetyParams {
        safety_pos_margin: 0.5,
        safety_k_position: 0.7,
    };

    println!(
        "Safety Pos Margin: {:?}, Safety K Position: {:?}",
        safety_params.safety_pos_margin, safety_params.safety_k_position
    );
}

pub fn angle_to_radian(angle: f32) -> f32 {
    angle * std::f32::consts::PI / 180.0
}

pub fn radian_to_angle(radian: f32) -> f32 {
    radian / std::f32::consts::PI * 180.0
}

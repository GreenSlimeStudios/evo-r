pub fn angle_to_radian(angle: f32) -> f32 {
    angle * std::f32::consts::PI / 180.0
}

pub fn radian_to_angle(radian: f32) -> f32 {
    let mut ang = radian / std::f32::consts::PI * 180.0;
    // if ang < 0.0 {
    //     ang = 180.0 - ang;
    // }
    ang
}

pub fn change_angle(angle: f32, degree: f32) -> f32 {
    let mut temp_angle = angle + degree;
    // if temp_angle < 0.0 {
    //     temp_angle = 360.0 - degree;
    // }
    // while temp_angle > 360.0 {
    //     temp_angle -= 360.0;
    // }
    temp_angle
}

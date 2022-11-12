use std::f32::consts::PI;

pub fn angle_to_radian_full(angle: f32) -> f32 {
    let rad = angle * PI / 180.0;
    rad
}

pub fn angle_to_radian(angle: f32) -> f32 {
    let mut rad = (angle * PI) / 180.0;
    if rad > PI {
        // rad = -(rad - PI);
        // rad -= PI;
        rad = -(PI + PI - rad);
    }
    rad
}

pub fn radian_to_angle(radian: f32) -> f32 {
    let mut new_rad = radian.clone();
    let mut is_fipped = false;
    if new_rad < 0.0 {
        // new_rad = PI - new_rad;
        new_rad = PI - new_rad;
        is_fipped = true;
    }
    let mut ang = (new_rad * 180.0) / PI;
    if is_fipped {
        ang = 540.0 - ang;
    }
    // if ang < 0.0 {
    //     ang = 180.0 - ang;
    // }

    ang
}

pub fn change_angle(angle: f32, degree: f32) -> f32 {
    let mut temp_angle = angle + degree;
    while temp_angle > 360.0 {
        temp_angle -= 360.0;
    }
    while temp_angle < 0.0 {
        temp_angle += 360.0;
    }
    temp_angle
}

// pub fn to_1_360_range(angle:f32)->f32{
//     let ang = angle.round() +

// }
pub fn is_angle_between(angle: f32, limit_a: f32, limit_b: f32) -> bool {
    // print!("{}: {} {} ", angle, limit_a, limit_b);
    if limit_a < limit_b {
        return limit_a <= angle && angle <= limit_b;
    } else {
        return limit_a <= angle || angle <= limit_b;
    }
}
pub fn is_surpasing_limit_1(angle: f32, limit_a: f32, limit_b: f32) -> bool {
    if limit_a < limit_b {
        return limit_a > angle;
    } else {
        return limit_a > angle;
    }
}
pub fn is_surpasing_limit_2(angle: f32, limit_a: f32, limit_b: f32) -> bool {
    if limit_a < limit_b {
        return angle > limit_b;
    } else {
        return angle > limit_b;
    }
}

#[test]
fn change_angle_test() {
    assert_eq!(change_angle(0.0, 10.0), 10.0);
    assert_eq!(change_angle(0.0, -10.0), 350.0);
    assert_eq!(change_angle(10.0, -20.0), 350.0);
    assert_eq!(change_angle(360.0, 20.0), 20.0);
    assert_eq!(change_angle(270.0, 100.0), 10.0);
}
#[test]
fn radian_angle_test() {
    assert_eq!(90.0, radian_to_angle(angle_to_radian(90.0)));
    assert_eq!(180.0, radian_to_angle(angle_to_radian(180.0)));
    assert_eq!(220.0, radian_to_angle(angle_to_radian(220.0)));
    assert_eq!(270.0, radian_to_angle(angle_to_radian(270.0)));
    assert_eq!(5.0, radian_to_angle(angle_to_radian(5.0)));

    assert_eq!(1.3, angle_to_radian(radian_to_angle(1.3)));
    assert_eq!(1.0, angle_to_radian(radian_to_angle(1.0)));
    assert_eq!(-1.2999997, angle_to_radian(radian_to_angle(-1.3)));
    assert_eq!(2.0, angle_to_radian(radian_to_angle(2.0)));
    assert_eq!(-0.5, angle_to_radian(radian_to_angle(-0.5)));
    assert_eq!(-2.7, angle_to_radian(radian_to_angle(-2.7)));
}
// #[test]
// fn radian_to_angle_test() {
//     assert_eq!(90.0, radian_to_angle(PI / 2.0));
//     assert_eq!(135.0, radian_to_angle(PI * 3.0 / 4.0));
//     assert_eq!(349.97324, radian_to_angle(-0.175));
// }
#[test]
fn angle_to_radian_test() {
    assert_eq!(-0.8726649, angle_to_radian(310.0));
    assert_eq!(-0.8726649, angle_to_radian(310.0));
    assert_eq!(-2.9670596, angle_to_radian(190.0));
    assert_eq!(1.3962635, angle_to_radian(80.0));
    assert_eq!(2.9670596, angle_to_radian(170.0));
    assert_eq!(0.87266463, angle_to_radian(50.0));
    assert_eq!(-2.4434612, angle_to_radian(220.0));
}
#[test]
fn radian_to_angle_test() {
    assert_eq!(310.0, radian_to_angle(-0.8726649));
    assert_eq!(190.00003, radian_to_angle(-2.9670596));
    assert_eq!(80.0, radian_to_angle(1.3962635));
    assert_eq!(169.99998, radian_to_angle(2.9670596));
    assert_eq!(50.0, radian_to_angle(0.87266463));
    assert_eq!(220.0, radian_to_angle(-2.4434612));
    assert_eq!(269.93106, radian_to_angle(-1.572));
}

#[test]
fn is_angle_between_test() {
    assert_eq!(true, is_angle_between(300.0, 180.0, 10.0));
    assert_eq!(false, is_angle_between(70.0, 80.0, 10.0));
    assert_eq!(true, is_angle_between(70.0, 10.0, 80.0));
    assert_eq!(false, is_angle_between(150.0, 170.0, 130.0));
    assert_eq!(true, is_angle_between(10.0, 300.0, 30.0));
    assert_eq!(true, is_angle_between(100.0, 300.0, 250.0));
}

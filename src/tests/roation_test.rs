use crate::quaternions::Quaterion;
use glam::{self, vec3, Mat3, Vec3};

const PI = 3.14159;

#[test]
fn rotate_quaterion() {
    let v  = Quaterion::new(0.0, 0.0,0.0,1.0);
    let q = Quaterion::new(0.707107,0.0,0.707107,0.0);
    let conugate = q.conjugate();
    let r = q*v*conugate;
    let exp_r = Quaterion::new(0.0, 1.0, 0.0, 0.0);
    assert_eq!(r,exp_r);
}

#[test]
fn rotate_matrix() {
    let v  = vec3(0.0, 0.0, 1.0);
    let m= Mat3::from_euler(glam::EulerRot::XYZ, 0.0, 0.5 *3.14159, 0.0);
    let r = m*v;
    let exp_r = vec3(1.0,0.0,0.0);
    assert_eq!(r,exp_r);
}




fn apply_tyny_roation_q(v:Quaterion<f32>,x:f32,y:f32,z:f32)-> Quaterion<f32>{
    let q = Quaterion::formEE(z, y, z); 
    let conugate = q.conjugate();
    let r = q*v*conugate;
    return r;
}

use crate::quaternions::Quaterion;
use glam::{self, vec3, Mat3, Vec3};

const PI:f32 = 3.14159;

//#[test]
fn rotate_quaterion() {
    let v  = Quaterion::new(0.0, 0.0,0.0,1.0);
    let q = Quaterion::new(0.707107,0.0,0.707107,0.0);
    let conugate = q.conjugate();
    let r = q*v*conugate;
    let exp_r = Quaterion::new(0.0, 1.0, 0.0, 0.0);
    assert_eq!(r,exp_r);
}

//#[test]
fn rotate_matrix() {
    let v  = vec3(0.0, 0.0, 1.0);
    let m= Mat3::from_euler(glam::EulerRot::XYZ, 0.0, 0.5 *3.14159, 0.0);
    let r = m*v;
    let exp_r = vec3(1.0,0.0,0.0);
    assert_eq!(r,exp_r);
}

#[test]
fn quaterion_vs_matrcies() {
    let mut matrix_soums:u128 =0;
    let mut quaterion_soums:u128 =0;
    let mut quat_s_vc:Vec<u128> = vec![];
    let mut mat_s_vc:Vec<u128> = vec![];
    let mut overall_error_m:f64 = 0.0;
    let mut overall_error_q:f64 = 0.0;
    let accuracy = 100000;
    for i in 0..accuracy{
        let nr = 998;
        println!("===quatrions====");
        let q_res = rotate_quateriontnr(nr);
        quat_s_vc.push(q_res.0);
        overall_error_q += q_res.1;
        println!("====matrix====");
        let m_res = rotate_matrix_nr(nr);
        mat_s_vc.push(m_res.0);
        overall_error_m += m_res.1;
        quaterion_soums = (quat_s_vc.clone().iter().sum::<u128>() / (quat_s_vc.len() as u128));
        matrix_soums = (mat_s_vc.clone().iter().sum::<u128>() /(mat_s_vc.len() as u128));
    println!("qauterions {} mateix {}", quaterion_soums, matrix_soums);

    }
    println!("============");
    println!("qauterions {} mateix {}", quaterion_soums, matrix_soums);
    println!("qauterions err {} mateix err {}", overall_error_q, overall_error_m);
    assert_eq!(1,2);
}


fn rotate_quateriontnr(nr:i32)-> (u128, f64){
    use std::time::Instant;
    let now = Instant::now();
    let mut v = Quaterion::new(0.0,0.0,0.0,1.0);
    let mut intendet_result = Quaterion::new(0.0, 1.0, 0.0, 0.0);
    let angle = (PI/2.0)/(nr as f32);
    for i in 0..nr{
        v = apply_tyny_roation_q(v, 0.0, angle ,0.0 );
    }
    println!("{:#?}",v);
    let elapsed = now.elapsed();
    let diff = ((intendet_result.real - v.real).abs() + (intendet_result.i_part - v.i_part).abs() + (intendet_result.j_part - v.j_part).abs() + (intendet_result.k_part - v.k_part).abs()) as f64;
    println!("Elapsed: {:.2?}", elapsed);
    return (elapsed.as_nanos(),diff);
}

fn rotate_matrix_nr(nr:i32)-> (u128,f64){
    use std::time::Instant;
    let now = Instant::now();
    let mut v = vec3(0.0,0.0,1.0);
    let intdet_res = vec3(1.0, 0.0, 0.0);
    let angle = (PI/2.0)/(nr as f32);
    for i in 0..nr{
        v =apply_tyny_roation_m(v, 0.0, angle ,0.0 );
    }
    println!("{:#?}",v);
    let elapsed = now.elapsed();
    let diff = v.distance(intdet_res) as f64;
    println!("Elapsed: {:.2?}", elapsed);
    return (elapsed.as_nanos(), diff);
}


fn apply_tyny_roation_q(v:Quaterion<f32>,x:f32,y:f32,z:f32)-> Quaterion<f32>{
    let q = Quaterion::formEE(z, y, z); 
    let conugate = q.conjugate();
    let r = q*v*conugate;
    return r;
}
fn apply_tyny_roation_m(v:Vec3,x:f32,y:f32,z:f32)-> Vec3{
    let m = Mat3::from_euler(glam::EulerRot::XYZ, x, y, z);
    let r = m*v;
    return r;
}

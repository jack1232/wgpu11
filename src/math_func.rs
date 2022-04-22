#![allow(dead_code)]
use cgmath::*;

pub fn sphere_position(r:f32, theta:Deg<f32>, phi:Deg<f32>) ->[f32; 3]{
    let x = r*theta.sin()*phi.cos();
    let y = r*theta.cos();
    let z = -r*theta.sin()*phi.sin();
    [x, y, z]
}
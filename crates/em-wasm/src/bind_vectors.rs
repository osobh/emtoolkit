//! WASM bindings for em-vectors: vector operations, scalar/vector fields.

use wasm_bindgen::prelude::*;
use em_core::coordinates::Vector3;
use em_vectors::{vector_ops, scalar_field, vector_field, differential_ops};

#[wasm_bindgen]
pub fn vector_add(ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64) -> JsValue {
    let r = vector_ops::vector_add(Vector3::new(ax, ay, az), Vector3::new(bx, by, bz));
    serde_wasm_bindgen::to_value(&r).unwrap()
}

#[wasm_bindgen]
pub fn vector_cross(ax: f64, ay: f64, az: f64, bx: f64, by: f64, bz: f64) -> JsValue {
    let r = vector_ops::cross_product(Vector3::new(ax, ay, az), Vector3::new(bx, by, bz));
    serde_wasm_bindgen::to_value(&r).unwrap()
}

#[wasm_bindgen]
pub fn vector_project(vx: f64, vy: f64, vz: f64, rx: f64, ry: f64, rz: f64) -> JsValue {
    let r = vector_ops::project(Vector3::new(vx, vy, vz), Vector3::new(rx, ry, rz));
    serde_wasm_bindgen::to_value(&r).unwrap()
}

#[wasm_bindgen]
pub fn scalar_field_2d(preset: &str, x_min: f64, x_max: f64, y_min: f64, y_max: f64, nx: usize, ny: usize) -> JsValue {
    let field = match preset {
        "paraboloid" => scalar_field::ScalarFieldPreset::Paraboloid,
        "saddle" => scalar_field::ScalarFieldPreset::Saddle,
        "sincos" => scalar_field::ScalarFieldPreset::SinCos,
        "inverse_r" => scalar_field::ScalarFieldPreset::InverseR,
        "gaussian" => scalar_field::ScalarFieldPreset::Gaussian,
        _ => scalar_field::ScalarFieldPreset::Paraboloid,
    };
    let grid = scalar_field::sample_2d(field, (x_min, x_max), (y_min, y_max), 0.0, nx, ny);
    serde_wasm_bindgen::to_value(&grid).unwrap()
}

#[wasm_bindgen]
pub fn vector_field_2d(preset: &str, x_min: f64, x_max: f64, y_min: f64, y_max: f64, nx: usize, ny: usize) -> JsValue {
    let field = match preset {
        "radial_outward" => vector_field::VectorFieldPreset::RadialOutward,
        "rotation" => vector_field::VectorFieldPreset::Rotation2D,
        "uniform_x" => vector_field::VectorFieldPreset::UniformX,
        "radial_inward" => vector_field::VectorFieldPreset::RadialInward,
        _ => vector_field::VectorFieldPreset::RadialOutward,
    };
    let grid = vector_field::sample_2d(field, (x_min, x_max), (y_min, y_max), 0.0, nx, ny);
    serde_wasm_bindgen::to_value(&grid).unwrap()
}

#[wasm_bindgen]
pub fn numerical_gradient(preset: &str, x: f64, y: f64, z: f64) -> JsValue {
    let field = match preset {
        "paraboloid" => scalar_field::ScalarFieldPreset::Paraboloid,
        "saddle" => scalar_field::ScalarFieldPreset::Saddle,
        "gaussian" => scalar_field::ScalarFieldPreset::Gaussian,
        _ => scalar_field::ScalarFieldPreset::Paraboloid,
    };
    let f = |x, y, z| field.evaluate(x, y, z);
    let g = differential_ops::gradient(&f, x, y, z, 1e-5);
    let exact = field.gradient_exact(x, y, z);
    let result = serde_json::json!({
        "numerical": { "x": g.x, "y": g.y, "z": g.z },
        "exact": { "x": exact.x, "y": exact.y, "z": exact.z },
    });
    serde_wasm_bindgen::to_value(&result).unwrap()
}

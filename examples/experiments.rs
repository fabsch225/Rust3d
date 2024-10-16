#[allow(unused)]

use rust3d::math::matrix::MatrixND;
use rust3d::engine::utils::transformation::PI;
use rust3d::math::vector::NVector;

fn dot(v1: &Vec<f64>, v2: &Vec<f64>) -> f64 {
    v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum()
}

fn norm(v: &Vec<f64>) -> f64 {
    dot(v, v).sqrt()
}

fn gram_schmidt(v1: &Vec<f64>, v2: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let mut v1_unit = v1.clone();
    let v1_norm = norm(&v1);
    v1_unit.iter_mut().for_each(|x| *x /= v1_norm);

    let mut v2_proj = v2.clone();
    let proj_coeff = dot(&v1_unit, &v2);
    v2_proj.iter_mut()
        .zip(v1_unit.iter())
        .for_each(|(v2_i, v1_u)| *v2_i -= proj_coeff * v1_u);

    let v2_norm = norm(&v2_proj);
    v2_proj.iter_mut().for_each(|x| *x /= v2_norm);

    (v1_unit, v2_proj)
}

fn rotation_matrix(v1: Vec<f64>, v2: Vec<f64>, angle: f64) -> Vec<Vec<f64>> {
    let n = v1.len();
    assert_eq!(v2.len(), n, "Both vectors must have the same dimension.");

    // Orthonormalize v1 and v2 using Gram-Schmidt
    let (v1_orth, v2_orth) = gram_schmidt(&v1, &v2);

    // Start with the identity matrix
    let mut rotation = vec![vec![0.0; n]; n];
    for i in 0..n {
        rotation[i][i] = 1.0;
    }

    // Apply 2D rotation in the plane spanned by v1_orth and v2_orth
    let cos_theta = angle.cos();
    let sin_theta = angle.sin();

    for i in 0..n {
        for j in 0..n {
            rotation[i][j] += cos_theta * v1_orth[i] * v1_orth[j]
                + sin_theta * v2_orth[i] * v1_orth[j]
                - sin_theta * v1_orth[i] * v2_orth[j]
                + cos_theta * v2_orth[i] * v2_orth[j];
        }
    }

    rotation
}


pub fn main() {
    let m = MatrixND {rows: 2, cols: 2, data: vec![2., 4., 0., 1.]};
    let vec = m.get_col_vector(0);
    let vec2 = m.get_row_vector(0);
    vec2.print();
    vec.print();
    let tm = MatrixND::translation_matrix(&vec);
    tm.print();

    let xy_plane = MatrixND {rows: 3, cols: 1, data: vec![0.,
                                                          1.,
                                                          0.,]};
    let matrix = MatrixND::aguilera_perez_single_thread(&xy_plane, PI / 2., 3);
    matrix.print();
    let mut vec1  = NVector{n: 3, x: vec![0., 1., 0.]};

    vec1.print();
    vec1 = matrix.multiply_nvector(&vec1);
    vec1.print();
    /*
    let v1 = vec![1.0, 0.0, 0.0];
    let v2 = vec![0.0, 1.0, 0.0];
    let angle = std::f64::consts::PI / 2.0; // Rotate by 45 degrees

    let matrix = rotation_matrix(v1, v2, angle);
    println!("Rotation matrix: {:?}", matrix);*/
}
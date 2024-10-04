#![allow(unused)]

pub mod engine {
    pub mod camera;
    pub mod proj_camera;
    pub mod pathtracing;
    pub mod raymarching;
    pub mod utils {
        pub mod anker_label;
        pub mod rendering;
        pub mod transformation;
        pub mod renderung_ui;
    }
    
    pub mod polytree {
        pub mod poly_tree;
        pub mod poly_tree_element;
        pub mod poly_tree_utils;
    }
}

pub mod geometry {
    pub mod poly_shape;
    pub mod face;
    pub mod point;
    pub mod quad;
    pub mod sphere;
    pub mod line;
    pub mod nface;
}

pub mod math {
    pub mod graph;
    pub mod matrix;

    pub mod matrix_utils {
        pub mod nmatrix_algorithms;
        pub mod nmatrix_creation;
        pub mod nmatrix_operations;
        pub mod nmatrix_rotations;

        pub mod three_d {
            pub mod utils;
        }
    }

    pub mod vector;
    pub mod functions;
    pub mod utils {
        pub mod graph_utils;
        pub mod assertion_utils;
    }

    pub mod optimization {
        pub mod float_ops;
    }
}
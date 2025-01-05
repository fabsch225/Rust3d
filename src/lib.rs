#![allow(unused)]
extern crate core;

pub mod engine {
    pub mod camera;
    pub mod projection_camera;
    pub mod pathtracing;
    pub mod raymarching;
    pub mod lighting;
    pub mod projection {
        pub mod projection;
        pub mod raster;
    }
    pub mod utils {
        pub mod anker_label;
        pub mod rendering;
        pub mod transformation;
        pub mod rendering_ui;
        pub mod js_canvas;
        pub mod virtual_canvas;
        pub mod raster_sorting;
    }
    pub mod simplex3d_sphere_tree {
        pub mod poly_tree;
        pub mod poly_tree_element;
        pub mod poly_tree_utils;
    }
    pub mod drawing {
        pub mod drawing;
        pub mod lines;
        pub mod circles;
    }

}

pub mod geometry {
    pub mod simplex3d;
    pub mod face;
    pub mod vector3;
    pub mod point;
    pub mod cube;
    pub mod quad;
    pub mod sphere;
    pub mod line;
    pub mod nface;
    pub mod nline;
    pub mod d2 {
        pub mod circle;
        pub mod line2d;
    }
}

pub mod math {
    pub mod graph;
    pub mod matrix;

    pub mod matrix_utils {
        pub mod nmatrix_algorithms;
        pub mod nmatrix_creation;
        pub mod nmatrix_operations;
        pub mod nmatrix_rotations;

        pub mod d3 {
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
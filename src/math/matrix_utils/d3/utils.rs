/*mat4x4 Matrix_MakeProjection(float fFovDegrees, float fAspectRatio, float fNear, float fFar)
{
    float fFovRad = 1.0f / tanf(fFovDegrees * 0.5f / 180.0f * 3.14159f);
    mat4x4 matrix;
    matrix.m[0][0] = fAspectRatio * fFovRad;
    matrix.m[1][1] = fFovRad;
    matrix.m[2][2] = fFar / (fFar - fNear);
    matrix.m[3][2] = (-fFar * fNear) / (fFar - fNear);
    matrix.m[2][3] = 1.0f;
    matrix.m[3][3] = 0.0f;
    return matrix;
}*/
use crate::math::matrix::NMatrix;

impl NMatrix {
    /// Creates a Projecting Matrix for 3D Graphics
    /// * `fov` - field of view in radians
    /// * `aspect_ratio` - aspect ratio
    /// * `near` - NEAR value
    /// * `far` - FAR value
    pub fn projection3d2d(fov: f64, aspect_ratio: f64, near: f64, far: f64) -> Self {
        let mut mat = NMatrix::new(4, 4);
        mat.set(0, 0, aspect_ratio * fov);
        mat.set(1, 1, fov);
        mat.set(2, 2, far / (far - near));
        mat.set(3, 2, (-far * near) / (far - near));
        mat.set(2, 3, 1.);
        mat
    }
}
# Plan for implementing Voxel Graphics

- Implement struct for world
    - this may include generation functions (a la minecraft chunk generation) -> perlin noise (?)
    - For now, single color voxels
- Implement Renderable Trait for this struct
  - Implement raymarching dor Voxels
- Implement 2 Major Optimizations
  1. Only Render Voxels that are on the surface
  2. Implement a spacial Tree (similar to the 3d sphere tree)
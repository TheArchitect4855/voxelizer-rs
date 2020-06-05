# voxelizer-rs
A Rust port of karimnaaji's C Voxelizer

This is a Rust port of karimnaaji's mesh voxelizer written in C. It is simple to use and has been carefully ported to Rust to give the most seamless experience possible.

## Examples
1. Voxelizing a mesh
```
let mesh = Mesh::new(vertices, colours, normals, indices);
let voxelized = mesh.voxelize(voxel_size, precision);
```
2. Voxelizing a mesh into a point cloud
```
let mesh = Mesh::new(vertices, colours, normals, indices);
let pointcloud = mesh.voxelize_pointcloud(voxel_size, precision);
```
3. Voxelizing a mesh into a texture (colour array).
```
let mesh = Mesh::new(vertices, colours, normals, indices);
let voxels = mesh.voxelize_texture(width, height, depth);
```

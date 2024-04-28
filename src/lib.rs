extern crate libc;

mod bindings;
use bindings::*;

#[derive(std::fmt::Debug)]
pub struct Vertex {
	pub v: [f32;3]
}

#[derive(std::fmt::Debug)]
pub struct Vector3 {
	pub x: f32,
	pub y: f32,
	pub z: f32
}

#[derive(std::fmt::Debug)]
pub struct Colour {
	pub r: f32,
	pub g: f32,
	pub b: f32
}

pub struct Mesh {
	pub vertices: Vec<Vertex>,
	pub colours: Vec<Colour>,
	pub normals: Vec<Vector3>,
	pub indices: Vec<u32>
}

pub struct PointCloud {
	pub vertices: Vec<Vertex>,
	pub colours: Vec<Colour>
}

impl Mesh {
	pub fn voxelize_pointcloud(&self, voxel_size: Vector3, precision: f32) -> PointCloud {
		let vx_mesh = self.allocate();
		let vx_pointcloud = unsafe { vx_voxelize_pc(vx_mesh, voxel_size.x, voxel_size.y, voxel_size.z, precision) };
		let pointcloud = PointCloud::from_vx(vx_pointcloud);
		unsafe { vx_point_cloud_free(vx_pointcloud) };
		pointcloud
	}
	
	pub fn voxelize(&self, voxel_size: Vector3, precision: f32) -> Mesh {
		let vx_mesh = self.allocate();
		let vx_mesh = unsafe { vx_voxelize(vx_mesh, voxel_size.x, voxel_size.y, voxel_size.z, precision) };
		let mesh = Mesh::from_vx(vx_mesh);
		unsafe { vx_mesh_free(vx_mesh) };
		mesh
	}
	
	pub fn voxelize_texture(&self, width: u32, height: u32, depth: u32) -> Vec<u32> {
		let mesh = self.allocate();
		let texture = unsafe { vx_voxelize_snap_3dgrid(mesh, width, height, depth) };
		
		let mut tex: Vec<u32> = Vec::new();
		let volume = width * height * depth;
		for i in 0..volume {
			let t = unsafe { *texture.offset(i as isize) };
			tex.push(t);
		}

		unsafe { libc::free(texture as *mut libc::c_void); }
		tex
	}
	
	pub fn new(vertices: Vec<Vertex>, colours: Vec<Colour>, normals: Vec<Vector3>, indices: Vec<u32>) -> Mesh {
		Mesh {
			vertices,
			colours,
			normals,
			indices
		}
	}
	
	fn allocate(&self) -> *mut vx_mesh {
		let mesh = unsafe { 
			match self.colours.len() > 0 {
				true => vx_mesh_alloc(self.vertices.len() as i32, self.indices.len() as i32),
				false => vx_color_mesh_alloc(self.vertices.len() as i32, self.indices.len() as i32)
			}
		};
		
		unsafe {
			let vertices = (*mesh).vertices;
			for i in 0..self.vertices.len() {
				let v = vx_vertex { v: self.vertices[i].v };
				vertices.offset(i as isize).write(v);
			}
			
			let colours = (*mesh).colors;
			for i in 0..self.colours.len() {
				let colour = &self.colours[i];
				let c = vx_color { r: colour.r, g: colour.g, b: colour.b };
				colours.offset(i as isize).write(c);
			}
			
			let normals = (*mesh).normals;
			for i in 0..self.normals.len() {
				let normal = &self.normals[i];
				let n = vx_vec3 { x: normal.x, y: normal.y, z: normal.z };
				normals.offset(i as isize).write(n);
			}
			
			let indices = (*mesh).indices;
			let normalindices = (*mesh).normalindices;
			for i in 0..self.indices.len() {
				let index = self.indices[i];
				indices.offset(i as isize).write(index);
				
				normalindices.offset(i as isize).write(index);
			}
			
			(*mesh).nindices = self.indices.len();
			(*mesh).nvertices = self.vertices.len();
			(*mesh).nnormals = self.normals.len();
		}
		
		mesh
	}
	
	fn from_vx(mesh: *mut vx_mesh) -> Mesh {
		let mut vertices: Vec<Vertex> = Vec::new();
		let mut colours: Vec<Colour> = Vec::new();
		let mut normals: Vec<Vector3> = Vec::new();
		let mut indices: Vec<u32> = Vec::new();
		
		unsafe {
			for i in 0..(*mesh).nvertices {
				let v = (*mesh).vertices.offset(i as isize);
				vertices.push(Vertex::new((*v).v));
				
				let c = (*mesh).colors.offset(i as isize);
				colours.push(Colour::new((*c).r, (*c).g, (*c).b));
			}
			
			for i in 0..(*mesh).nnormals {
				let n = (*mesh).normals.offset(i as isize);
				normals.push(Vector3::new((*n).x, (*n).y, (*n).z));
			}
			
			for i in 0..(*mesh).nindices {
				let ind = (*mesh).indices.offset(i as isize);
				indices.push(*ind);
			}
		}
		
		Mesh::new(vertices, colours, normals, indices)
	}
}

impl Vertex {
	pub fn new(v: [f32;3]) -> Vertex {
		Vertex { v }
	}
}

impl Colour {
	pub fn new(r: f32, g: f32, b: f32) -> Colour {
		Colour { r, g, b }
	}
}

impl Vector3 {
	pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
		Vector3 { x, y, z }
	}
}

impl PointCloud {
	fn from_vx(pointcloud: *mut vx_point_cloud) -> PointCloud {
		let mut vertices: Vec<Vertex> = Vec::new();
		let mut colours: Vec<Colour> = Vec::new();
		unsafe {
			for i in 0..(*pointcloud).nvertices {
				let v = (*pointcloud).vertices.offset(i as isize);
				vertices.push(Vertex::new((*v).v));
				
				let c = (*pointcloud).colors.offset(i as isize);
				colours.push(Colour::new((*c).r, (*c).g, (*c).b));
			}
		}
		
		PointCloud { vertices, colours }
	}
}

#[test]
fn test_voxelize_pointcloud() {
	let mesh = Mesh::new(
		vec![Vertex::new([-0.5, 0.5, 0.0]), Vertex::new([0.5, 0.5, 0.0]), Vertex::new([0.5, -0.5, 0.0]), Vertex::new([-0.5, 0.5, 0.0])],
		Vec::new(),
		vec![Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0)],
		vec![0, 1, 3, 1, 2, 3]
	);
	
	let pointcloud = mesh.voxelize_pointcloud(Vector3::new(0.2, 0.2, 0.2), 1.0);
	println!("Point Cloud Vertices: {:?}", pointcloud.vertices);
	println!("Point Cloud Colours: {:?}", pointcloud.colours);
}

#[test]
fn test_voxelize() {
	let mesh = Mesh::new(
		vec![Vertex::new([-0.5, 0.5, 0.0]), Vertex::new([0.5, 0.5, 0.0]), Vertex::new([0.5, -0.5, 0.0]), Vertex::new([-0.5, 0.5, 0.0])],
		Vec::new(),
		vec![Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0)],
		vec![0, 1, 3, 1, 2, 3]
	);
	
	let voxels = mesh.voxelize(Vector3::new(0.2, 0.2, 0.2), 1.0);
	println!("Voxel Mesh Vertices: {:?}", voxels.vertices);
	println!("Voxel Mesh Colours: {:?}", voxels.colours);
	println!("Voxel Mesh Normals: {:?}", voxels.normals);
	println!("Voxel Mesh Indices: {:?}", voxels.indices);
}

#[test]
fn test_voxelize_texture() {
	let mesh = Mesh::new(
		vec![Vertex::new([-0.5, 0.5, 0.0]), Vertex::new([0.5, 0.5, 0.0]), Vertex::new([0.5, -0.5, 0.0]), Vertex::new([-0.5, 0.5, 0.0])],
		Vec::new(),
		vec![Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0), Vector3::new(0.0, 0.0, 1.0)],
		vec![0, 1, 3, 1, 2, 3]
	);
	
	let voxels = mesh.voxelize_texture(4, 4, 4);
	println!("Voxels: {:?}", voxels);
}
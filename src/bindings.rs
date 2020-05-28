extern crate cty;

#[repr(C)]
pub struct vx_vertex {
	pub v: [cty::c_float;3]
}

#[repr(C)]
pub struct vx_vec3 {
	pub x: cty::c_float,
	pub y: cty::c_float,
	pub z: cty::c_float
}

#[repr(C)]
pub struct vx_color {
	pub r: cty::c_float,
	pub g: cty::c_float,
	pub b: cty::c_float
}

#[repr(C)]
pub struct vx_mesh {
	pub vertices: *mut vx_vertex,
	pub colors: *mut vx_color,
	pub normals: *mut vx_vec3,
	pub indices: *mut cty::c_uint,
	pub normalindices: *mut cty::c_uint,
	pub nindices: cty::size_t,
	pub nvertices: cty::size_t,
	pub nnormals: cty::size_t
}

#[repr(C)]
pub struct vx_point_cloud {
	pub vertices: *mut vx_vertex,
	pub colors: *mut vx_color,
	pub nvertices: cty::size_t
}

extern "C" {
	pub fn vx_voxelize_pc(mesh: *const vx_mesh, 
		voxelsizex: cty::c_float,
		voxelsizey: cty::c_float,
		voxelsizez: cty::c_float,
		precision: cty::c_float) -> *mut vx_point_cloud;

	pub fn vx_voxelize(mesh: *const vx_mesh, 
		voxelsizex: cty::c_float,
		voxelsizey: cty::c_float,
		voxelsizez: cty::c_float,
		precision: cty::c_float) -> *mut vx_mesh;

	pub fn vx_voxelize_snap_3dgrid(mesh: *const vx_mesh,
		width: cty::c_uint,
		height: cty::c_uint,
		depth: cty::c_uint) -> *mut cty::c_uint;

	pub fn vx_mesh_alloc(nvertices: cty::c_int, nindices: cty::c_int) -> *mut vx_mesh;
	pub fn vx_color_mesh_alloc(nvertices: cty::c_int, nindices: cty::c_int) -> *mut vx_mesh;
	pub fn vx_mesh_free(mesh: *mut vx_mesh);
	pub fn vx_point_cloud_free(pointcloud: *mut vx_point_cloud);
}
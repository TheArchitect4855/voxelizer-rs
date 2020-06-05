extern crate libc;

#[repr(C)]
pub struct vx_vertex {
	pub v: [libc::c_float;3]
}

#[repr(C)]
pub struct vx_vec3 {
	pub x: libc::c_float,
	pub y: libc::c_float,
	pub z: libc::c_float
}

#[repr(C)]
pub struct vx_color {
	pub r: libc::c_float,
	pub g: libc::c_float,
	pub b: libc::c_float
}

#[repr(C)]
pub struct vx_mesh {
	pub vertices: *mut vx_vertex,
	pub colors: *mut vx_color,
	pub normals: *mut vx_vec3,
	pub indices: *mut libc::c_uint,
	pub normalindices: *mut libc::c_uint,
	pub nindices: libc::size_t,
	pub nvertices: libc::size_t,
	pub nnormals: libc::size_t
}

#[repr(C)]
pub struct vx_point_cloud {
	pub vertices: *mut vx_vertex,
	pub colors: *mut vx_color,
	pub nvertices: libc::size_t
}

impl Drop for vx_mesh {
	fn drop(&mut self) {
		unsafe {
			vx_mesh_free(self);
		}
	}
}

impl Drop for vx_point_cloud {
	fn drop(&mut self) {
		unsafe {
			vx_point_cloud_free(self);
		}
	}
}

extern "C" {
	pub fn vx_voxelize_pc(mesh: *const vx_mesh, 
		voxelsizex: libc::c_float,
		voxelsizey: libc::c_float,
		voxelsizez: libc::c_float,
		precision: libc::c_float) -> *mut vx_point_cloud;

	pub fn vx_voxelize(mesh: *const vx_mesh, 
		voxelsizex: libc::c_float,
		voxelsizey: libc::c_float,
		voxelsizez: libc::c_float,
		precision: libc::c_float) -> *mut vx_mesh;

	pub fn vx_voxelize_snap_3dgrid(mesh: *const vx_mesh,
		width: libc::c_uint,
		height: libc::c_uint,
		depth: libc::c_uint) -> *mut libc::c_uint;

	pub fn vx_mesh_alloc(nvertices: libc::c_int, nindices: libc::c_int) -> *mut vx_mesh;
	pub fn vx_color_mesh_alloc(nvertices: libc::c_int, nindices: libc::c_int) -> *mut vx_mesh;
	pub fn vx_mesh_free(mesh: *mut vx_mesh);
	pub fn vx_point_cloud_free(pointcloud: *mut vx_point_cloud);
}
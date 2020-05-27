mod bindings;

#[test]
fn test_vx_voxelize_pc() {
	unsafe {
		let mesh = bindings::vx_mesh_alloc(16, 16);
		let _pointcloud = bindings::vx_voxelize_pc(mesh, 1.0, 1.0, 1.0, 1.0);
		//I'm not totally sure how to check if this stuff is actually valid, so we're just going to do nothing.
	}
}
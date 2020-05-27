extern crate cc;

fn main() {
	cc::Build::new()
		.file("voxelizer.c")
		.compile("vxc");
}
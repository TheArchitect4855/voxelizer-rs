extern crate cc;

fn main() {
	cc::Build::new()
		.file("voxelizer.c")
		.define("VOXELIZER_IMPLEMENTATION", None)
		.compile("vxc");
}
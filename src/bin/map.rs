use futures::executor::block_on;
use papariki::geometry::LonLat;
use papariki::globe::Globe;

fn main() {
	let mut globe = Globe::new("");
	let tile = block_on(globe.get_tile(0, 0, 1));
	let verts: Vec<f32> = tile.vertices();

	println!("Hello, world! {:?}", verts);
}

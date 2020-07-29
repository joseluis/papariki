use crate::mesh::Mesh;
use crate::protos::vector_tile::mod_Tile::Feature;
use nalgebra as na;
use std::f32::consts::PI;

#[derive(Debug, Clone)]
pub struct LonLat(na::Point2<f32>);

impl LonLat {
	pub fn new(lon: f32, lat: f32) -> Self {
		Self(na::Point2::new(lon, lat))
	}
}

impl Feature {
	pub fn to_mesh(&self) -> Mesh {
		Mesh {
			vertices: vec![],
			triangles: vec![],
		}
	}
}

// Convert a lon/lat into a 3D cartesian point
fn lonlat_to_point(ll: &na::Point2<f32>) -> na::Point3<f32> {
	let rad = 1.03;
	let lon = (ll.x).to_radians() as f32;
	let lat = (ll.y - 90.0).to_radians() as f32;

	na::Point3::new(
		-rad * lat.sin() * lon.sin(),
		-rad * lat.cos(),
		rad * lat.sin() * lon.cos(),
	)
}

// Convert a Mercator coordinate, between (0.0, 0.0) and (1.0, 1.0), into a lon/lat
fn pixel_to_lonlat(p: &na::Point2<f32>, zoom: f32) -> na::Point2<f32> {
	let tile_size = 1.0f32;

	let c = 0.5 * tile_size * 2.0_f32.powi(zoom as i32);
	let bc = c / 360.0;
	let cc = c / (2.0 * PI);

	let e = c / 2.0;
	let lon = (p.x - e) / bc;
	let g = (p.y - e) / -cc;
	let lat = (2.0f32 * g.exp().atan() - 0.5 * PI).to_degrees();

	na::Point2::new(lon, lat)
}

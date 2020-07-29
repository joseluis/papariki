mod renderer;
use renderer::WebGlRenderer;

use crate::globe::Globe;
use crate::tile::Tile;
use js_sys::{self, Array};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::{closure::Closure, JsValue};
use wasm_bindgen_futures::{future_to_promise, spawn_local};
use web_sys::{self, HtmlElement};

// Imported from JS land
#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace=console)]
	pub fn log(s: &str);
}

pub fn now() -> f64 {
	web_sys::window().unwrap().performance().unwrap().now()
}

// Export to JS land
#[wasm_bindgen]
pub struct Environment {
	globe: Rc<RefCell<Globe>>,
	renderer: Rc<RefCell<WebGlRenderer>>,
	animate_callback: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
}

#[wasm_bindgen]
pub fn attach(container: &HtmlElement, token: &str) -> Environment {
	let mut env = Environment {
		globe: Rc::new(RefCell::new(Globe::new(token))),
		renderer: Rc::new(RefCell::new(WebGlRenderer::new(1024, 768))),
		animate_callback: Rc::new(RefCell::new(None)),
	};
	// Add renderer to the DOM
	env.renderer.borrow_mut().attach(container);

	// Run on next tick
	let tile_promise = future_to_promise({
		let globe = env.globe.clone();
		let renderer = env.renderer.clone();
		async move {
			let mut globe = globe.borrow_mut();
			let zoom = 1;
			let n = 2_i32.pow(zoom);

			for y in 0..n {
				for x in 0..n {
					let tile = globe.get_tile(x, y, zoom as i32).await;
					let mut renderer = renderer.borrow_mut();
					renderer.add_mesh(tile.mesh());
				}
			}
			Ok(true.into())
		}
	});

	// Run every frame
	let closure = {
		let animate = env.animate_callback.clone();
		let renderer = env.renderer.clone();
		Closure::wrap(Box::new(move || {
			let animate = animate.borrow();
			if let Ok(mut renderer) = renderer.try_borrow_mut() {
				renderer.tick();
				renderer.draw();
			} else {
				log("Failed to tick renderer");
			}
			if let Some(callback) = animate.as_ref() {
				let window = web_sys::window().unwrap();
				let animation_id = window
					.request_animation_frame(callback.as_ref().unchecked_ref())
					.unwrap();
			}
		}) as Box<dyn FnMut()>)
	};

	let window = web_sys::window().unwrap();
	let animation_id = window
		.request_animation_frame(closure.as_ref().unchecked_ref())
		.unwrap();

	*env.animate_callback.borrow_mut() = Some(closure);

	env
}
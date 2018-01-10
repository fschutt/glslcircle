#[macro_use]
extern crate glium;

use glium::{Display, Program};
use glium::glutin::{self, Event, WindowEvent, ControlFlow};

pub const CIRCLE_VERTEX_SHADER: &str = include_str!("circle.vert");
pub const CIRCLE_FRAGMENT_SHADER: &str = include_str!("circle.frag");

fn main() {

    let window_builder = glutin::WindowBuilder::new()
        .with_title("circletest")
        .with_dimensions(800, 600);
    let context_builder = glutin::ContextBuilder::new()
        .with_srgb(false);
    let mut events_loop = glutin::EventsLoop::new();

    let display = Display::new(
        window_builder,
        context_builder,
        &events_loop,
    ).unwrap();

    let circle_shader = Program::from_source(&display, CIRCLE_VERTEX_SHADER, CIRCLE_FRAGMENT_SHADER, None).unwrap();

    events_loop.run_forever(|e| {
    	 match e {
    	 	Event::WindowEvent { event, window_id } => {
    	 		match event {
	    	 		WindowEvent::Closed => ControlFlow::Break,
	    	 		WindowEvent::Resized(_, _) => {
	    	 			draw_circles(&display, &circle_shader);
	    	 			ControlFlow::Continue
	    	 		},
	    	 		_ => ControlFlow::Continue,
    	 		}
    	 	},
    	 	_ => ControlFlow::Continue,
    	 }
    });
}

#[derive(Copy, Clone, Debug)]
struct VertexPoint {
	x: f32,
	y: f32,
}

implement_vertex!(VertexPoint, x, y);

fn draw_circles(display: &Display, shader: &Program) {
	use glium::{VertexBuffer};
	use glium::draw_parameters::DrawParameters;
	use glium::Surface;
	use glium::index::PrimitiveType;

	let vbuf = VertexBuffer::<VertexPoint>::empty(display, 3).unwrap();
	let ibuf = glium::index::IndicesSource::NoIndices{ primitives: PrimitiveType::TrianglesList };
	let mut frame = display.draw();
	frame.clear_color(1.0, 1.0, 1.0, 1.0);
	frame.draw(&vbuf, ibuf, shader, &uniform!{ },
				&DrawParameters::default()).unwrap();
	frame.finish().unwrap();
}
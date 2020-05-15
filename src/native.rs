// wengwengweng

use glutin::event_loop::ControlFlow;
use glutin::event::WindowEvent as WEvent;
use glutin::event::DeviceEvent as DEvent;
use glutin::event::TouchPhase;
use glutin::event::ElementState;

use crate::*;
use input::*;

pub fn handle_winit_event(e: Event) -> ControlFlow {
	return ControlFlow::Poll;

}


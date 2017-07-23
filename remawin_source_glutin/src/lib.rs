extern crate remawin;
extern crate glutin;
extern crate time;
extern crate remawin_glutin_mapper;

use remawin::raw::{RawInputSource, RawInput};
use remawin::types::WindowPosition;
use remawin_glutin_mapper::{NextData, GlutinEventMapper};

pub struct GlutinInputSource {
    events_loop : Option<glutin::EventsLoop>,
    last_cursor_position : Option<WindowPosition>,
    current_size : (f64, f64),
}

impl GlutinInputSource {
    pub fn new(events_loop: Option<glutin::EventsLoop>, current_size : (f64, f64)) -> GlutinInputSource {
        GlutinInputSource {
            events_loop : events_loop,
            last_cursor_position : None,
            current_size: current_size
        }
    }
}

impl RawInputSource for GlutinInputSource {

    fn process(&mut self) -> Vec<RawInput> {
        let mut events = Vec::new();
        self.events_loop.as_mut().unwrap().poll_events(|event| {
            events.push(event);
        });

        let mut next = NextData {
            size : self.current_size.clone(),
            cursor_position : self.last_cursor_position.clone()
        };
        let raw = events.iter().flat_map(|e| GlutinEventMapper::process_event(&e, &mut next)).collect();
        self.last_cursor_position = next.cursor_position;
        self.current_size = next.size;

        raw
    }

}

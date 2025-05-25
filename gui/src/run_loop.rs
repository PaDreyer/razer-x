#![allow(unused)]

use tao::{
    event::Event,
    event_loop::{EventLoop, EventLoopWindowTarget, ControlFlow, EventLoopBuilder},
};

pub struct RunLoop<T> where T: 'static{
    event_loop: EventLoop<T>,
}

impl<T> RunLoop<T> where T: 'static {
    pub fn new() -> Self {
        let event_loop = EventLoopBuilder::<T>::with_user_event()
            .build();

        RunLoop {
            event_loop,
        }
    }

    pub fn create_proxy(&self) -> tao::event_loop::EventLoopProxy<T> {
        self.event_loop.create_proxy()
    }

    pub fn run<F>(self, mut event_handler: F) -> !
        where F: FnMut(Event<'_, T>, Box<dyn FnOnce()>) + 'static
    {
        self.event_loop.run(move |event, event_loop_window_target, control_flow | {
            *control_flow = ControlFlow::Wait;

            let control_flow_ptr = control_flow as *mut _;
            event_handler(event, Box::new(move || {
                println!("Exiting");
                unsafe { *control_flow_ptr = ControlFlow::Exit };
            }));
        });
    }
}
use instant::Instant;
use render::State;
use wasm_bindgen::prelude::*;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod render;

pub struct Store {
    pub local_storage: web_sys::Storage,
}

fn save_to_localstore(name: &str, item: &str) {
    let window = web_sys::window();
    if let Ok(Some(local_storage)) = window.unwrap().local_storage() {
        let mut store = Store { local_storage };
        store.local_storage.set_item(name, item);
    }
}

fn load_from_localstore(item: &str) -> Option<String> {
    let window = web_sys::window();
    if let Ok(Some(local_storage)) = window.unwrap().local_storage() {
        let store = Store { local_storage };
        if let Ok(Some(value)) = store.local_storage.get_item(item) {
            Some(value)
        } else {
            None
        }
    } else {
        None
    }
}

fn load_obj(object: &str, material: &str) {
    let decoded_obj = base64::decode(object);
    let decoded_mtl = base64::decode(material);
    let m = tobj::load_obj_buf(
        &mut decoded_obj.as_ref().unwrap().as_slice(),
        true,
        |p| match p.file_name().unwrap().to_str().unwrap() {
            "Paddle.mtl" => tobj::load_mtl_buf(&mut decoded_mtl.as_ref().unwrap().as_slice()),
            _ => unreachable!(),
        },
    );
    log::info!("object: {:?}", m);
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut state = State::new(&window).await;

    let size = window.inner_size();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::NewEvents(_) => {}
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                state.resize(size);
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                state.render();
                log::info!("update");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,

            Event::DeviceEvent { device_id, event } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {
                log::info!("Suspended");
            }
            Event::Resumed => {
                log::info!("Resumed");
                window.request_redraw();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawEventsCleared => {}
            Event::LoopDestroyed => {}
            _ => {}
        }
    });
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");

        let event_loop = EventLoop::new();
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_title("Window");
        let window = builder.build(&event_loop).unwrap();

        use winit::platform::web::WindowExtWebSys;

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");

        let event_loop = EventLoop::new();
        let window = winit::window::Window::new(&event_loop).unwrap();
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}

use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;

async fn run() {
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    let adapter = wgpu::Instance::new(wgpu::BackendBit::PRIMARY)
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();

    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    log::info!("Adapter Information: {:?}", adapter);

    let event_loop = EventLoop::new();
    let mut builder = winit::window::WindowBuilder::new();
    builder = builder.with_title("Window");
    let window = builder.build(&event_loop).unwrap();
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    {
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
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg_attr(target_arch = "wasm32", allow(unused_variables))]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
}

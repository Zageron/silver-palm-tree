use std::io::BufRead;

use wasm_bindgen::prelude::*;
use winit::event_loop::EventLoop;

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
        let mut store = Store { local_storage };
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

    // let paddle_asset = b"# Blender v2.91.2 OBJ File: 'Paddle.blend'
    // # www.blender.org
    // mtllib Paddle.mtl
    // o Plane
    // v -1.000000 0.000000 0.000000
    // v 1.000000 0.000000 0.000000
    // v -1.000000 0.000000 -1.000000
    // v 1.000000 0.000000 -1.000000
    // vt 0.000000 0.000000
    // vt 1.000000 0.000000
    // vt 1.000000 1.000000
    // vt 0.000000 1.000000
    // vn 0.0000 1.0000 0.0000
    // usemtl None
    // s off
    // f 1/1/1 2/2/1 4/3/1 3/4/1
    // ";

    // let paddle_mtl = b"# Blender MTL File: 'Paddle.blend'
    // # Material Count: 1

    // newmtl None
    // Ns 500
    // Ka 0.8 0.8 0.8
    // Kd 0.8 0.8 0.8
    // Ks 0.8 0.8 0.8
    // d 1
    // illum 2
    // ";

    // save_to_localstore("paddle_asset", base64::encode(paddle_asset).as_str());
    // save_to_localstore("paddle_mtl", base64::encode(paddle_mtl).as_str());

    let paddle_asset = load_from_localstore("paddle_asset");
    let paddle_mtl = load_from_localstore("paddle_mtl");
    load_obj(paddle_asset.unwrap().as_str(), paddle_mtl.unwrap().as_str());
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

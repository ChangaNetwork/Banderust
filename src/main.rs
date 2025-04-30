mod app;

use app::App;
pub mod abstractions {
    pub mod response;
    pub mod agent;
}

pub mod components {
    pub mod session;
    pub mod agent;
}

fn main() {
    yew::Renderer::<App>::new().render();
    wasm_logger::init(wasm_logger::Config::default());
}

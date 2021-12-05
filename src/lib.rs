use vertigo_browserdriver::start_browser_app;

mod app;
mod state;
mod list;

#[no_mangle]
pub fn start_application() {
    // Run component in rendering driver
    start_browser_app(state::State::new, app::render);
}

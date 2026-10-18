#[allow(dead_code)]
fn needs_sync<T: Sync>() {}

#[test]
fn event_loop_proxy_send() {
    #[allow(dead_code)]
    fn is_send<T: 'static + Send + Sync>() {
        // ensures that `mario_window::EventLoopProxy<T: Send>` implements `Sync`
        needs_sync::<mario_window::event_loop::EventLoopProxy<T>>();
    }
}

#[test]
fn window_sync() {
    // ensures that `mario_window::Window` implements `Sync`
    needs_sync::<mario_window::window::Window>();
}

#[test]
fn window_builder_sync() {
    needs_sync::<mario_window::window::WindowAttributes>();
}

#[test]
fn custom_cursor_sync() {
    needs_sync::<mario_window::window::CustomCursorSource>();
    needs_sync::<mario_window::window::CustomCursor>();
}

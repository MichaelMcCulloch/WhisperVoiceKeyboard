pub(crate) fn init() {
    android_logger::init_once(android_logger::Config::default().with_min_level(log::Level::Trace));
}
pub(crate) fn uninit() {}

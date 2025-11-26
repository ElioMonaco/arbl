pub fn setup_logger() {
    // Set up logging functionality for UEFI (e.g., logging to console)
    uefi::logger::init();
}
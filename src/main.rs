use egui_pocketbook;
use epi::NativeOptions;
use std::{panic, thread};
use std::fs::File;
use std::io::Write;
use inkview_sys;
use inkview_sys::c_api;
use std::ffi::CString;
use core::time;
use inkview_sys::c_api::Event;
use std::sync::{Mutex, Arc};
use backtrace::Backtrace;


fn main() {
    unsafe{
        inkview_sys::c_api::OpenScreen();
    }
    //let h: Arc<Mutex<dyn inkview_sys::EventHandler>> = Arc::new(Mutex::new(EventHandler{}));


    //inkview_sys::main(&h);

    //let mut file = File::create("/var/log/panic_log.txt");

    //let ten_millis = time::Duration::from_millis(10000);

    //thread::sleep(ten_millis);

    //file.write_all(b"Panic happens!");
    panic::set_hook(Box::new(|info| {
        let backtrace = Backtrace::new();
        //let mut file = File::open("/var/tmp/panic_log.txt").unwrap();
        //file.write_all(format!("Panic! {:?} {:?}", info, backtrace).as_ref());
        unsafe{
            inkview_sys::c_api::OpenScreen();
        }
        inkview_sys::message(c_api::Icon::ERROR, "Panic", &format!("panic: {:?} {:?}", info, backtrace), 5);
        unsafe { c_api::CloseApp(); }

    }));

    let app = egui_template::TemplateApp::default();
    let native_options = NativeOptions::default();
    egui_pocketbook::start(Box::new(app), native_options);
}

fn main() {
    let (connection, screen_num) = xcb::Connection::connect(None).unwrap();

    let root = connection
        .get_setup()
        .roots()
        .nth(screen_num as usize)
        .unwrap()
        .root();

    loop {
        let status = format!("{}", chrono::Local::now().format("%a %d/%m/%Y %T"));

        let c_string = std::ffi::CString::new(status).unwrap();

        xcb::change_property(
            &connection,
            xcb::PROP_MODE_REPLACE as u8,
            root,
            xcb::ATOM_WM_NAME,
            xcb::ATOM_STRING,
            8,
            c_string.as_bytes(),
        );

        connection.flush();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

// Unlicense — cochranblock.org
// iOS entry point: called from Swift via @_silgen_name
//! Only compiled when target_os = "ios"

#[cfg(target_os = "ios")]
#[no_mangle]
pub extern "C" fn od_start_server(data_dir: *const std::ffi::c_char, port: i32) {
    let data_dir = unsafe { std::ffi::CStr::from_ptr(data_dir) }
        .to_str()
        .unwrap_or("/tmp/oakilydokily")
        .to_string();
    let port = port as u16;

    std::env::set_var("OAKILYDOKILY_DATA_DIR", &data_dir);
    std::env::set_var("PORT", port.to_string());
    std::env::set_var("BIND", "127.0.0.1");

    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    rt.block_on(async {
        let pool = crate::waiver::init_pool(std::path::Path::new(&data_dir))
            .await
            .expect("init pool");
        let app = crate::web::router::router(crate::AppState {
            s0: pool,
            s1: None,
            s2: crate::web::forge::new_cache(),
        });
        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
        )
        .await
        .expect("serve");
    });
}

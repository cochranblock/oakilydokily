// Unlicense — cochranblock.org
// JNI bridge for Android: starts the oakilydokily server from Java/Kotlin
//! Only compiled when target_os = "android" and feature = "android"

#![allow(non_snake_case)]

#[cfg(all(target_os = "android", feature = "android"))]
use jni::JNIEnv;
#[cfg(all(target_os = "android", feature = "android"))]
use jni::objects::JClass;

/// Called from ServerService.startServer(dataDir, port) via JNI
#[cfg(all(target_os = "android", feature = "android"))]
#[no_mangle]
pub extern "system" fn Java_org_oakilydokily_ServerService_startServer(
    mut env: JNIEnv,
    _class: JClass,
    data_dir: jni::objects::JString,
    port: jni::sys::jint,
) {
    let data_dir: String = env.get_string(&data_dir)
        .expect("invalid data_dir string")
        .into();
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

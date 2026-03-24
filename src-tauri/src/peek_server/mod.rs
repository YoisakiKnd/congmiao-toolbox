use axum::{
    routing::{get, post},
    Router,
    response::IntoResponse,
    Json,
    http::StatusCode,
    response::Response,
    body::Body,
    extract::Query,
};
#[derive(serde::Deserialize)]
pub struct AudioControl {
    pub mute: Option<bool>,
    pub volume: Option<u32>,
}

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use tokio::sync::oneshot;
use screenshots::Screen;
use sysinfo::System;
use std::io::Cursor;
use tower_http::cors::{Any, CorsLayer};

mod audio;


pub static PRIVACY_MODE: AtomicBool = AtomicBool::new(false);
pub static IS_RUNNING: AtomicBool = AtomicBool::new(false);
pub static PRIVACY_IMAGE_PATH: OnceLock<Mutex<Option<String>>> = OnceLock::new();

fn get_privacy_image_path() -> &'static Mutex<Option<String>> {
    PRIVACY_IMAGE_PATH.get_or_init(|| Mutex::new(None))
}


static SHUTDOWN_TX: OnceLock<Mutex<Option<oneshot::Sender<()>>>> = OnceLock::new();

fn get_tx() -> &'static Mutex<Option<oneshot::Sender<()>>> {
    SHUTDOWN_TX.get_or_init(|| Mutex::new(None))
}

pub async fn run_server() {
    if IS_RUNNING.load(Ordering::SeqCst) {
        return;
    }

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
        
    let app = Router::new()
        .route("/api/status", get(status_handler))
        .route("/api/screenshot", get(screenshot_handler))
        .route("/api/privacy", get(get_privacy).post(set_privacy))
        .route("/api/audio", get(get_audio_handler).post(post_audio_handler))

        .layer(cors);

    let (tx, rx) = oneshot::channel();
    *get_tx().lock().unwrap() = Some(tx);

    IS_RUNNING.store(true, Ordering::SeqCst);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    tokio::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                rx.await.ok();
            })
            .await
            .unwrap();
        
        IS_RUNNING.store(false, Ordering::SeqCst);
        *get_tx().lock().unwrap() = None;
    });
}

pub fn stop_server() {
    if let Some(tx) = get_tx().lock().unwrap().take() {
        let _ = tx.send(());
    }
    IS_RUNNING.store(false, Ordering::SeqCst);
}

async fn status_handler() -> impl IntoResponse {
    let mut sys = System::new_all();
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    
    let cpu = sys.global_cpu_usage();
    let mem = sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0;
    
    let audio_info = audio::get_audio_status();
    
    Json(serde_json::json!({
        "status": "success",
        "cpu": cpu,
        "memory": {
            "usedPercent": mem
        },
        "audio": audio_info
    }))
}

async fn get_audio_handler() -> impl IntoResponse {
    Json(audio::get_audio_status())
}

async fn post_audio_handler(Json(payload): Json<AudioControl>) -> impl IntoResponse {
    if let Some(m) = payload.mute {
        audio::set_mute(m);
    }
    if let Some(v) = payload.volume {
        audio::set_volume(v);
    }
    Json(audio::get_audio_status())
}

async fn screenshot_handler() -> impl IntoResponse {
    let screens = Screen::all().unwrap();
    if screens.is_empty() {
        return Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap();
    }
    
    let screen = screens.first().unwrap();
    let image_res = screen.capture();
    
    if image_res.is_err() {
        return Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap();
    }
    
    let image = image_res.unwrap();
    let is_privacy = PRIVACY_MODE.load(Ordering::SeqCst);
    
    let buffer = if is_privacy {
        // Check if we have a custom privacy image
        let custom_path = get_privacy_image_path().lock().unwrap().clone();
        if let Some(path) = custom_path {
            if let Ok(img) = image::open(path) {
                let mut bytes: Vec<u8> = Vec::new();
                img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg).unwrap();
                return Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "image/jpeg")
                    .header("Access-Control-Allow-Origin", "*")
                    .body(Body::from(bytes))
                    .unwrap();
            }
        }

        let mut dynamic_img = image::DynamicImage::ImageRgba8(image::RgbaImage::from_raw(image.width(), image.height(), image.into_raw()).unwrap());
        let w = std::cmp::max(dynamic_img.width() / 4, 1);
        let h = std::cmp::max(dynamic_img.height() / 4, 1);
        dynamic_img = dynamic_img.resize_exact(w, h, image::imageops::FilterType::Nearest);
        // Intensity increased for "80% blur"
        let blurred = image::imageops::blur(&dynamic_img, 30.0);
        
        let mut bytes: Vec<u8> = Vec::new();
        blurred.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg).unwrap();
        bytes
    } else {
        let mut dynamic_img = image::DynamicImage::ImageRgba8(image::RgbaImage::from_raw(image.width(), image.height(), image.into_raw()).unwrap());
        let w = std::cmp::max(dynamic_img.width() / 2, 1);
        let h = std::cmp::max(dynamic_img.height() / 2, 1);
        dynamic_img = dynamic_img.resize_exact(w, h, image::imageops::FilterType::Lanczos3);

        let mut bytes: Vec<u8> = Vec::new();
        dynamic_img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg).unwrap();
        bytes
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/jpeg")
        .header("Access-Control-Allow-Origin", "*")
        .body(Body::from(buffer))
        .unwrap()
}

async fn get_privacy() -> impl IntoResponse {
    let current = PRIVACY_MODE.load(Ordering::SeqCst);
    Json(serde_json::json!({ "enabled": current }))
}

async fn set_privacy() -> impl IntoResponse {
    let current = PRIVACY_MODE.load(Ordering::SeqCst);
    PRIVACY_MODE.store(!current, Ordering::SeqCst);
    Json(serde_json::json!({ "enabled": !current }))
}

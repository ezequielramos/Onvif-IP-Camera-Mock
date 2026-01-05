mod circle;
mod ffmpeg;
mod render;

use ab_glyph::FontArc;
use circle::CircleState;
use render::render_frame;
use tokio::io::AsyncWriteExt;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let width = 480;
    let height = 320;
    let fps = 15;

    let rtsp_url = std::env::var("RTSP_URL").unwrap_or("rtsp://127.0.0.1:8554/cam1".into());

    // fonte
    let font_data = include_bytes!("../assets/FreeMono.ttf");
    let font = FontArc::try_from_slice(font_data).expect("invalid font");

    let mut state = CircleState::new(width, height);

    // inicia ffmpeg
    let mut ffmpeg_stdin = ffmpeg::start_ffmpeg(&rtsp_url, width, height, fps).await;

    println!("Streaming to {}", rtsp_url);

    loop {
        state.update();

        let frame = render_frame(&state, width, height, &font);

        // envia frame cru (RGB24)
        if ffmpeg_stdin.write_all(frame.as_raw()).await.is_err() {
            eprintln!("ffmpeg error");
            break;
        }

        sleep(Duration::from_millis(1000 / fps as u64)).await;
    }
}

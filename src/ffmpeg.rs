use std::process::Stdio;
use tokio::process::{ChildStdin, Command};

pub async fn start_ffmpeg(rtsp_url: &str, width: u32, height: u32, fps: u32) -> ChildStdin {
    let size = format!("{}x{}", width, height);
    let fps = fps.to_string();

    let mut child = Command::new("ffmpeg")
        .args([
            "-loglevel",
            "warning",
            // input raw frames
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgb24",
            "-s",
            &size,
            "-r",
            &fps,
            "-i",
            "-",
            // encoder
            "-c:v",
            "libx264",
            "-preset",
            "ultrafast",
            "-tune",
            "zerolatency",
            "-pix_fmt",
            "yuv420p",
            // RTSP
            "-f",
            "rtsp",
            "-rtsp_transport",
            "tcp",
            rtsp_url,
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("falha ao iniciar ffmpeg");

    child.stdin.take().unwrap()
}

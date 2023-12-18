use chrono::Duration;
use std::path::Path;

pub fn get_video_duration<P: AsRef<Path>>(path: P) -> Result<Duration, ffmpeg::Error> {
    let context = ffmpeg::format::input(&path)?;

    Ok(Duration::microseconds(context.duration()))
}

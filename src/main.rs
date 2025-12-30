use ndi::FrameFormatType;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[cfg(target_os = "linux")]
fn main() -> color_eyre::Result<()> {
    use scap::capturer::Capturer;

    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    if !scap::is_supported() {
        color_eyre::eyre::bail!("Platform not supported");
    }

    if !scap::has_permission() {
        tracing::info!("Permission not granted. Requesting permission...");
        if !scap::request_permission() {
            color_eyre::eyre::bail!("Permission denied");
        }
    }

    let targets = scap::get_all_targets();
    tracing::debug!("Targets: {:?}", targets);

    let options = scap::capturer::Options {
        fps: 60,
        target: None, // None captures the primary display
        show_cursor: false,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_1080p,
        crop_area: None,
        ..Default::default()
    };

    let mut capturer = Capturer::build(options)?;

    capturer.start_capture();

    ndi::initialize()?;

    let send = ndi::send::SendBuilder::new()
        .ndi_name("Desktop".to_string())
        .build()?;

    loop {
        let frame = capturer.get_next_frame()?;
        let scap::frame::Frame::BGRA(mut frame) = frame else {
            continue;
        };

        let ndi_frame = ndi::VideoData::from_buffer(frame.width, frame.height, ndi::FourCCVideoType::BGRA, 60, 1, FrameFormatType::Progressive, 0, frame.width * 4, None, &mut frame.data);
        send.send_video(&ndi_frame);
    }
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn main() -> color_eyre::Result<()> {
    use scap::capturer::Capturer;

    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    if !scap::is_supported() {
        color_eyre::eyre::bail!("Platform not supported");
    }

    if !scap::has_permission() {
        tracing::info!("Permission not granted. Requesting permission...");
        if !scap::request_permission() {
            color_eyre::eyre::bail!("Permission denied");
        }
    }

    let targets = scap::get_all_targets();
    tracing::debug!("Targets: {:?}", targets);

    let options = scap::capturer::Options {
        fps: 60,
        target: None, // None captures the primary display
        show_cursor: false,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_1080p,
        crop_area: None,
        ..Default::default()
    };

    let mut capturer = Capturer::build(options)?;

    capturer.start_capture();

    ndi::initialize()?;

    let send = ndi::send::SendBuilder::new()
        .ndi_name("Desktop".to_string())
        .build()?;

    loop {
        let frame = capturer.get_next_frame()?;
        let scap::frame::Frame::Video(scap::frame::VideoFrame::BGRA(mut frame)) = frame else {
            continue;
        };

        let ndi_frame = ndi::VideoData::from_buffer(frame.width, frame.height, ndi::FourCCVideoType::BGRA, 60, 1, FrameFormatType::Progressive, 0, frame.width * 4, None, &mut frame.data);
        send.send_video(&ndi_frame);
    }
}

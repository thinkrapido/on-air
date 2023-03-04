use nokhwa::{pixel_format::LumaFormat, utils::RequestedFormat, Camera, NokhwaError};
use on_air::{air, warning, warning_off, WHITE};

#[macro_use]
extern crate log;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    #[cfg(target_os = "linux")]
    on_air_warning_linux::start(main_loop)?;
    #[cfg(target_os = "macos")]
    on_air_warning_macos::start(main_loop)?;
    #[cfg(target_os = "windows")]
    on_air_warning_windows::start(main_loop)?;

    #[allow(unreachable_code)]
    Ok(())
}

fn main_loop() -> anyhow::Result<()> {
    loop {
        let mut is_streaming = false;

        {
            for idx in 0..4u32 {
                #[allow(clippy::single_match)]
                match Camera::new(
                    nokhwa::utils::CameraIndex::Index(idx),
                    RequestedFormat::new::<LumaFormat>(nokhwa::utils::RequestedFormatType::None),
                ) {
                    #[cfg(target_os = "windows")]
                    Ok(mut cam) => if let Err(NokhwaError::ReadFrameError(..)) = cam.frame() {
                        is_streaming = true;
                        break;
                    },
                    #[cfg(target_os = "unix")]
                    Err(NokhwaError::SetPropertyError { .. }) => {
                        is_streaming = true;
                        break;
                    }
                    _ => {}
                }
            }
        }

        info!("camera is {}", if is_streaming { "on" } else { "off" });

        if is_streaming {
            air!(warning(WHITE))?;
        } else {
            air!(warning_off())?;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

}
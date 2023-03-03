use nokhwa::{pixel_format::LumaFormat, utils::RequestedFormat, Camera, NokhwaError};
use on_air::{air, warning, warning_off, WHITE};

#[macro_use]
extern crate log;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    loop {
        let mut is_streaming = false;

        {
            for idx in 0..4u32 {
                match Camera::new(
                    nokhwa::utils::CameraIndex::Index(idx),
                    RequestedFormat::new::<LumaFormat>(nokhwa::utils::RequestedFormatType::None),
                ) {
                    #[cfg(target_os = "windows")]
                    Ok(mut cam) => match cam.frame() {
                        Err(NokhwaError::ReadFrameError(..)) => {
                            is_streaming = true;
                            break;
                        }
                        _ => {}
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

    #[allow(unreachable_code)]
    Ok(())
}

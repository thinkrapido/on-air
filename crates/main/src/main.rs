use nokhwa::{Camera, utils::RequestedFormat, pixel_format::{LumaFormat}, NokhwaError};
use on_air::{air, warning_off, WHITE, warning};

fn main() -> anyhow::Result<()>{

    loop {

        let mut is_streaming = false;
        for idx in 0..4u32 {
            if let Err(NokhwaError::SetPropertyError{..}) = Camera::new(nokhwa::utils::CameraIndex::Index(idx), RequestedFormat::new::<LumaFormat>(nokhwa::utils::RequestedFormatType::None)) {
                is_streaming = true;
                break;
            }
        }
        if is_streaming {
            air!(
                warning(WHITE)
            )?;
        }
        else {
            air!(
                warning_off()
            )?;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[allow(unreachable_code)]
    Ok(())
}

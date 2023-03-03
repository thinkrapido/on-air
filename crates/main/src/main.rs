use nokhwa::{Camera, utils::RequestedFormat, pixel_format::{LumaFormat}, NokhwaError};
//use on_air::{air, warning_off, WHITE, warning};
//use opencv::videoio::{VideoCaptureAPIs, VideoCapture, VideoCaptureTrait, CAP_ANY};

fn main() -> anyhow::Result<()>{

    loop {

        // {

        //     for idx in 0..1 {
        //         let mut cam = VideoCapture::new(idx, CAP_ANY).expect("failed to open video capture");
        //         // let stream = cam.open(idx, VideoCaptureAPIs::CAP_ANY as i32);
        //         // match stream {
        //         //     Ok(_) => println!("stream open for {idx}"),
        //         //     Err(err) => println!("failed for {idx} {err:?}"),
        //         // }
        //     }

        // }

        {
            let mut is_streaming = false;
            for idx in 0..1u32 {
                if let Err(NokhwaError::SetPropertyError{..}) = Camera::new(nokhwa::utils::CameraIndex::Index(idx), RequestedFormat::new::<LumaFormat>(nokhwa::utils::RequestedFormatType::None)) {
                    is_streaming = true;
                    println!("camera is on");
                    break;
                }
                else {
                    println!("camera is off");
                }
            }
       }
 

        // if is_streaming {
        //     air!(
        //         warning(WHITE)
        //     )?;
        // }
        // else {
        //     air!(
        //         warning_off()
        //     )?;
        // }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    #[allow(unreachable_code)]
    Ok(())
}

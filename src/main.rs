use rusb::{DeviceList, GlobalContext, DeviceHandle};





fn main() -> anyhow::Result<()> {
    
    let device_list = DeviceList::new()?;

    for device in device_list.iter() {

        let descriptor = device.device_descriptor()?;
        if let (0x5131, 0x2007) = (descriptor.vendor_id(), descriptor.product_id()) {

            let handle = device.open()?;

            println!("read");
            for i in 0..SIZE {
                result_output(i, read(&handle, i));
            }

            println!("write");
            for i in 0..SIZE {
                result_output(i, write(&handle, i));
            }

            println!("finished");

            break;
        }

    }

    Ok(())
}

const SIZE: usize = 64;

fn read(handle: &DeviceHandle<GlobalContext>, i: usize) -> anyhow::Result<()> {

    let endpoint = 0x82u8;
    let mut buf: [u8; SIZE] = [0; SIZE];

    let bytes_read = handle.read_bulk(
        endpoint, 
        &mut buf[..i], 
        std::time::Duration::from_secs(1))?;

    println!("bytes read {bytes_read}");

    Ok(())
}

fn write(handle: &DeviceHandle<GlobalContext>, i: usize) -> anyhow::Result<()> {

    let endpoint = 0x02u8;
    let buf: [u8; SIZE] = [0u8; SIZE];

    let bytes_written = handle.write_interrupt(
        endpoint, 
        &buf[..i], 
        std::time::Duration::from_secs(1))?;

    println!("bytes written {bytes_written}");

    Ok(())
}

fn result_output(i: usize, result: anyhow::Result<()>) {
    match result {
        Ok(_) => println!("success: {i}"),
        Err(err) => print!("{i:2}. {err:?}\t"),
    };
    if i % 4 == 3 { println!(""); }
}
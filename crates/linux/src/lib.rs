use tray_item::TrayItem;
use gtk;

pub fn start(fun: fn() -> anyhow::Result<()>) -> anyhow::Result<()> {

    std::thread::spawn(move || {
        anyhow::Ok(fun()?)
    });

    gtk::init().unwrap();

    let mut tray = TrayItem::new("Tray Example", "camera-large").map_err(anyhow::Error::from)?;

    tray.add_label("On Air Warning").map_err(anyhow::Error::from)?;

    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    }).unwrap();

    gtk::main();

    Ok(())
}
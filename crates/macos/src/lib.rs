use tray_item::TrayItem;

pub fn start(fun: fn() -> anyhow::Result<()>) -> anyhow::Result<()> {

    std::thread::spawn(move || {
        anyhow::Ok(fun()?)
    });

    let mut tray = TrayItem::new("On Air Warning", "").map_err(anyhow::Error::from)?;

    tray.add_label("On Air Warning").map_err(anyhow::Error::from)?;

    let mut inner = tray.inner_mut();
    inner.add_quit_item("Quit");
    inner.display();

    Ok(())
}
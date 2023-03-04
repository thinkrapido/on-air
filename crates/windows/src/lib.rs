use {std::sync::mpsc, tray_item::TrayItem};

enum Message {
    Quit,
}

pub fn start(fun: fn() -> anyhow::Result<()>) -> anyhow::Result<()> {

    std::thread::spawn(move || {
        anyhow::Ok(fun()?)
    });

    let mut tray = TrayItem::new("On Air Warning", "my-icon-name").map_err(anyhow::Error::from)?;

    tray.add_label("On Air Warning").map_err(anyhow::Error::from)?;

    let (tx, rx) = mpsc::channel();

    tray.add_menu_item("Quit", move || {
        println!("Quit");
        tx.send(Message::Quit).unwrap();
    }).map_err(anyhow::Error::from)?;

    loop {
        if let Ok(Message::Quit) = rx.recv() {
            break
        }
    }

    Ok(())
}

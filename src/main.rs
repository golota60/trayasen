use futures::future;

use idasen::get_instance_by_mac;
use tray_item::TrayItem;

// EE:4D:A2:34:E4:8F
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    gtk::init().unwrap();

    let desk = rt.spawn(get_instance_by_mac("EE:4D:A2:34:E4:8F"));
    let desk = rt.block_on(async { desk.await });
    // TODO: Find a better way of unwrapping this shit
    let desk = match desk {
        Ok(desk) => desk,
        Err(error) => panic!("Error while connecting to the desk: {:?}", error),
    };
    let desk = match desk {
        Ok(desk) => desk,
        Err(error) => panic!("Error while connecting to the desk2: {:?}", error),
    };

    // TODO: how accessorries icons work?
    let mut tray = TrayItem::new("Idasen desk tray", "accessories-calculator").unwrap();

    tray.add_label("Tray Label").unwrap();

    tray.add_menu_item("Hello", move || {
        // TODO: this doesn't execute - tokio should be able to fix this
        rt.block_on(async {
            println!("Trying to bring your idasen up!");
            desk.move_to(8000).await;
        });

        // .await?;
    })
    .unwrap();

    tray.add_menu_item("Quit", || {
        gtk::main_quit();
    })
    .unwrap();
    // test_func().await;

    gtk::main();
}

#![windows_subsystem = "windows"]

use notify_rust::Notification;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }

    app.set_tooltip(&"KeyCoach".to_string())?;
    app.set_icon_from_file("kc.ico")?;

    app.add_menu_item("Print a thing", |_| {
        println!("Printing a thing!");
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Add Menu Item", |window| {
        window.add_menu_item("Interior item", |_| {
            println!("what");
            Ok::<_, systray::Error>(())
        })?;
        window.add_menu_separator()?;
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Set Reminder", |_| {
        thread::spawn(|| {
            thread::sleep(Duration::from_secs(5)); // Wait for 5 seconds (adjust as needed)
            Notification::new()
                .appname("keyCoach")
                .summary("Reminder")
                .body("'Ctrl + C' copies things!")
                .show()
                .expect("Failed to send notification");
        });
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_separator()?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    println!("Waiting on message!");
    app.wait_for_message()?;

    Ok(())
}

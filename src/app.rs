use gio;
use gtk;
use gtk::prelude::*;
use std::env;

// TODO: Is this the correct format for GApplication IDs?
const APP_ID: &'static str = "jplatte.ruma_gtk";

/// State for the main thread.
///
/// It takes care of starting up the application and for loading and accessing the
/// UI.
pub struct App {
    /// GTK Application which runs the main loop.
    gtk_app: gtk::Application,
}

impl App {
    /// Create an App instance
    pub fn new() -> App {
        let gtk_app = gtk::Application::new(Some(APP_ID), gio::ApplicationFlags::empty())
            .expect("Failed to initialize GtkApplication");

        let gtk_builder = gtk::Builder::new_from_file("res/main_window.glade");

        let builder = gtk_builder.clone();
        gtk_app.connect_activate(move |app| {
            // Set up shutdown callback
            let window: gtk::Window = builder.get_object("main_window")
                .expect("Couldn't find main_window in ui file.");

            let app2 = app.clone();
            window.connect_delete_event(move |_, _| {
                app2.quit();
                Inhibit(false)
            });

            // Set up user popover
            let user_button: gtk::Button = builder.get_object("user_button")
                .expect("Couldn't find user_button in ui file.");

            let user_menu: gtk::Popover = builder.get_object("user_menu")
                .expect("Couldn't find user_menu in ui file.");

            user_button.connect_clicked(move |_| user_menu.show());

            // Associate window with the Application and show it
            window.set_application(Some(app));
            window.show_all();
        });

        App {
            gtk_app: gtk_app,
        }
    }

    pub fn run(self) {
        // Convert the args to a Vec<&str>.  Application::run requires argv as &[&str]
        // and envd::args() returns an iterator of Strings.
        let args = env::args().collect::<Vec<_>>();
        let args_refs = args.iter().map(|x| &x[..]).collect::<Vec<_>>();

        // Run the main loop.
        self.gtk_app.run(args_refs.len() as i32, &args_refs);
    }
}

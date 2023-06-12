use gtk::prelude::*;
use gtk::{Button, Label, Window, WindowType};

fn main() {
    // Initialize GTK and create a new window
    gtk::init().expect("Failed to initialize GTK.");
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Pathfinder Character Builder");
    window.set_default_size(400, 300);

    // Create the main layout for the window
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Add a label for the race selection
    let race_label = Label::new(Some("Select a Race:"));
    layout.add(&race_label);

    // Add buttons for each race
    let races = vec!["Human", "Elf", "Dwarf", "Gnome", "Halfling"];
    for race in races {
        let race_button = Button::new_with_label(race);
        layout.add(&race_button);

        // Connect a callback to the button click event
        race_button.connect_clicked(move |_| {
            // Perform actions when a race button is clicked
            println!("Race selected: {}", race);
            // You can continue implementing the other steps in similar fashion
        });
    }

    // Add the layout to the window and display it
    window.add(&layout);
    window.show_all();

    // Start the GTK main event loop
    gtk::main();
}


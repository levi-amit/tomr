mod cli;

use cursive::{
    views::{TextView, Dialog}
};


pub fn start_ui() -> Result<(), ()> {
    cli::start_cli()
}


#[allow(dead_code)]
fn start_cursive() -> Result<(), ()> {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(TextView::new("Hello! Press <q> to quit."));

    siv.add_layer(Dialog::text("Hello"));

    siv.run();
    
    Ok(())
}

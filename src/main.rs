mod ui;
mod dbg;

fn main() -> () {
    dbg::setup_dbg();
    ui::start_ui().ok();
}

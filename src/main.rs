mod app;

use app::RGreet;
use iced::application;

fn main() {
    let app = application(RGreet::boot, RGreet::update, RGreet::view);

    app.run().expect("Could not run app");
}

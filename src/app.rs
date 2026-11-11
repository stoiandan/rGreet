use std::net::Ipv4Addr;

use iced::Element;
use iced::widget::text;


#[derive(Clone, Copy)]
pub enum Message {
    
}

pub struct RGreet {
    pub hosts: Vec<Host>
}




struct Host {
    hostname: String,
    ip: Ipv4Addr
}


impl RGreet {

    pub fn boot() -> Self {
        RGreet { hosts: Vec::new() }
    }

    pub fn view(&self) -> Element<'_, Message> {
        text("Hello World")
            .into()
    }

    pub fn update(&mut self, message: Message) {
    }
}
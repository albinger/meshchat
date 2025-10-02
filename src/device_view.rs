use crate::device_view::ConnectionState::{Connected, Connecting, Disconnected, Disconnecting};
use crate::device_view::DeviceEvent::{
    ConnectedEvent, DeviceConnect, DeviceDisconnect, DisconnectedEvent,
};
use crate::{comms, Message, NavigationMessage};
use iced::widget::{button, container, text, Column};
use iced::{Element, Length, Task};
use meshtastic::utils::stream::BleId;

#[derive(Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting(BleId),
    Connected(BleId),
    Disconnecting(BleId),
}

#[derive(Debug, Clone)]
pub enum DeviceEvent {
    DeviceConnect(BleId),
    DeviceDisconnect(BleId),
    ConnectedEvent(BleId),
    DisconnectedEvent(BleId),
}

pub struct DeviceView {
    connection_state: ConnectionState,
}
async fn empty() {}

impl DeviceView {
    pub fn new() -> Self {
        Self {
            connection_state: Disconnected,
        }
    }

    pub fn connection_state(&self) -> ConnectionState {
        self.connection_state.clone()
    }

    /// Return a true value to show we can show the device view, false for main to decide
    pub fn update(&mut self, device_event: DeviceEvent) -> Task<Message> {
        match device_event {
            DeviceConnect(id) => {
                self.connection_state = Connecting(id.clone());
                Task::perform(comms::do_connect(id.clone()), |result| {
                    Message::Device(ConnectedEvent(result.unwrap()))
                })
            }
            DeviceDisconnect(id) => {
                self.connection_state = Disconnecting(id.clone());
                Task::perform(comms::do_disconnect(id.clone()), |result| {
                    Message::Device(DisconnectedEvent(result.unwrap()))
                })
            }
            ConnectedEvent(id) => {
                self.connection_state = Connected(id);
                Task::perform(empty(), |_| {
                    Message::Navigation(NavigationMessage::Connected)
                })
            }
            DisconnectedEvent(_) => {
                self.connection_state = Disconnected;
                Task::perform(empty(), |_| Message::Navigation(NavigationMessage::Back))
            }
        }
    }

    pub fn view(&self) -> Element<'static, Message> {
        let mut main_col = Column::new();

        main_col = main_col
            .push(button("<-- Back").on_press(Message::Navigation(NavigationMessage::Back)));

        match &self.connection_state {
            Disconnected => {
                main_col = main_col.push(text("disconnected"));
            }

            Connecting(id) => {
                main_col = main_col.push(text(format!("connecting to : {id}")));
            }

            Connected(id) => {
                main_col = main_col.push(text(format!("connected to : {id}")));
                main_col = main_col.push(
                    button("Disconnect").on_press(Message::Device(DeviceDisconnect(id.clone()))),
                );
            }
            Disconnecting(id) => {
                main_col = main_col.push(text(format!("disconnecting from : {id}")));
            }
        }

        let content = container(main_col)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Left);

        content.into()
    }
}

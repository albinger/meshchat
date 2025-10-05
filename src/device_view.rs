use crate::device_view::ConnectionState::{Connected, Connecting, Disconnected, Disconnecting};
use crate::device_view::DeviceEvent::{
    ConnectedEvent, DeviceConnect, DeviceDisconnect, DisconnectedEvent,
};
use crate::{comms, Message, NavigationMessage};
use iced::widget::{button, container, text, Column};
use iced::{Element, Length, Task};
use meshtastic::api::ConnectedStreamApi;
use meshtastic::packet::PacketReceiver;
use meshtastic::utils::stream::BleId;
use std::sync::Arc;

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
    ConnectedEvent(BleId, Arc<PacketReceiver>, Arc<ConnectedStreamApi>),
    DisconnectedEvent(BleId),
}

pub struct DeviceView {
    pub connection_state: ConnectionState,
    packet_receiver: Option<Arc<PacketReceiver>>,
    stream_api: Option<Arc<ConnectedStreamApi>>,
}
async fn empty() {}

impl DeviceView {
    pub fn new() -> Self {
        Self {
            connection_state: Disconnected,
            packet_receiver: None,
            stream_api: None,
        }
    }

    pub fn connected(&self) -> Option<BleId> {
        match &self.connection_state {
            Connected(id) => Some(id.clone()),
            _ => None,
        }
    }

    pub fn connection_state(&self) -> &ConnectionState {
        &self.connection_state
    }

    /// Return a true value to show we can show the device view, false for main to decide
    pub fn update(&mut self, device_event: DeviceEvent) -> Task<Message> {
        match device_event {
            DeviceConnect(id) => {
                self.connection_state = Connecting(id.clone());
                Task::perform(comms::do_connect(id.clone()), move |result| {
                    let (packet_receiver, stream_api) = result.unwrap();
                    Message::Device(ConnectedEvent(
                        id.clone(),
                        Arc::new(packet_receiver),
                        Arc::new(stream_api),
                    ))
                })
            }
            DeviceDisconnect(_) => {
                if let Some(id) = self.connected() {
                    self.disconnect(&id)
                } else {
                    Task::none() // TODO report an error?
                }
            }
            ConnectedEvent(id, packet_receiver, stream_api) => {
                self.packet_receiver = Some(packet_receiver);
                self.stream_api = Some(stream_api);
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

    pub fn disconnect(&mut self, id: &BleId) -> Task<Message> {
        self.connection_state = Disconnecting(id.clone());
        self.packet_receiver.take();
        let stream_api = self.stream_api.take().unwrap();
        Task::perform(
            comms::do_disconnect(id.clone(), Arc::into_inner(stream_api).unwrap()),
            |result| Message::Device(DisconnectedEvent(result.unwrap())),
        )
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

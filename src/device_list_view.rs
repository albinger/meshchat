use crate::device_view::ConnectionState;
use crate::device_view::ConnectionState::{Connected, Connecting, Disconnected, Disconnecting};
use crate::device_view::DeviceViewMessage::{ConnectRequest, DisconnectRequest};
use crate::discovery::DiscoveryEvent;
use crate::styles::chip_style;
use crate::Message::{Device, Navigation};
use crate::{name_from_id, Message, NavigationMessage};
use iced::widget::{button, container, text, Column, Row};
use iced::{Element, Length, Task};
use meshtastic::utils::stream::BleDevice;

#[derive(Default)]
pub struct DeviceListView {
    devices: Vec<BleDevice>,
}

async fn empty() {}

impl DeviceListView {
    pub fn update(&mut self, discovery_event: DiscoveryEvent) -> Task<Message> {
        match discovery_event {
            DiscoveryEvent::BLERadioFound(id) => self.devices.push(id),
            DiscoveryEvent::BLERadioLost(id) => self.devices.retain(|other_id| other_id != &id),
            DiscoveryEvent::Error(e) => {
                return Task::perform(empty(), move |_| {
                    Message::AppError("Discovery Error".to_string(), e.to_string())
                });
            }
        };

        Task::none()
    }

    pub fn header<'a>(&'a self, connection_state: &'a ConnectionState) -> Element<'a, Message> {
        match connection_state {
            Disconnected(_, _) => text("Disconnected").into(),
            Connecting(device) => {
                text(format!("Connecting to {}", device.name.as_ref().unwrap())).into()
            }
            Connected(device) => button(text(device.name.as_ref().unwrap()))
                .style(chip_style)
                .on_press(Navigation(NavigationMessage::DeviceView))
                .into(),
            Disconnecting(device) => text(format!(
                "Disconnecting from {}",
                device.name.as_ref().unwrap()
            ))
            .into(),
        }
    }

    pub fn view(&self, connection_state: &ConnectionState) -> Element<'static, Message> {
        let mut main_col = Column::new();
        // TODO add a scanning bar at the top
        // TODO add a scrollable area in case there are a lot of devices

        for device in &self.devices {
            let mut device_row = Row::new();
            device_row = device_row.push(text(name_from_id(device)));
            match &connection_state {
                Connected(connected_device) => {
                    if connected_device.eq(device) {
                        device_row = device_row.push(
                            button("Disconnect")
                                .on_press(Device(DisconnectRequest(device.clone())))
                                .style(chip_style),
                        );
                    }
                }
                // TODO maybe show an error against it if present?
                Disconnected(_id, _error) => {
                    device_row = device_row.push(
                        button("Connect")
                            .on_press(Device(ConnectRequest(device.clone(), None)))
                            .style(chip_style),
                    );
                }
                Connecting(connecting_device) => {
                    if connecting_device == device {
                        device_row = device_row.push(text("Connecting"));
                    }
                }
                Disconnecting(disconnecting_device) => {
                    if disconnecting_device == device {
                        device_row = device_row.push(text("Disconnecting"));
                    }
                }
            }
            main_col = main_col.push(device_row);
        }

        container(main_col)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Left)
            .into()
    }
}

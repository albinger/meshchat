use crate::device_view::DeviceEvent::{DeviceConnect, DeviceDisconnect};
use crate::discovery::DiscoveryEvent;
use crate::Message;
use crate::Message::Device;
use btleplug::platform::PeripheralId;
use iced::widget::{button, container, text, Column, Row};
use iced::{Element, Length, Task};
use std::collections::HashMap;

pub struct DeviceListView {
    devices: HashMap<PeripheralId, String>,
}

impl DeviceListView {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub fn update(&mut self, discovery_event: DiscoveryEvent) -> Task<Message> {
        match discovery_event {
            DiscoveryEvent::BLERadioFound(id, name) => self.devices.insert(id, name),
            DiscoveryEvent::BLERadioLost(id) => self.devices.remove(&id),
        };

        Task::none()
    }

    pub fn view(&self, connected_device: Option<&PeripheralId>) -> Element<'static, Message> {
        let mut main_col = Column::new();
        main_col = main_col.push(text("Scanning...Available devices:"));

        for (id, name) in &self.devices {
            let mut device_row = Row::new();
            let mut device_button = button(text(name.clone()));
            if let Some(connected_device) = connected_device {
                if connected_device == id {
                    device_row = device_row.push(device_button);
                    device_row = device_row
                        .push(button("Disconnect").on_press(Device(DeviceDisconnect(id.clone()))));
                }
            } else {
                device_button = device_button.on_press(Device(DeviceConnect(id.clone())));
                device_row = device_row.push(device_button);
            }
            main_col = main_col.push(device_row);
        }

        let content = container(main_col)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Left);

        content.into()
    }
}

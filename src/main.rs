//! MeshChat is an iced GUI app that uses the meshtastic "rust" crate to discover and control
//! meshtastic compatible radios connected to the host running it

use crate::config::{load_config, save_config, Config};
use crate::device_list_view::DeviceListView;
use crate::device_view::ConnectionState::Connected;
use crate::device_view::{DeviceView, DeviceViewMessage};
use crate::discovery::{ble_discovery, DiscoveryEvent};
use crate::Message::{AppError, Device, Discovery, Exit, Navigation, NewConfig, WindowEvent};
use iced::widget::{text, Column, Row};
use iced::{window, Element, Event, Subscription, Task};
use meshtastic::utils::stream::BleId;
use std::cmp::PartialEq;

mod channel_view;
mod config;
mod device_list_view;
mod device_subscription;
mod device_view;
mod discovery;
// mod router;

#[derive(PartialEq, Default)]
enum View {
    #[default]
    DeviceList,
    Device,
}

#[derive(Debug, Clone)]
pub enum NavigationMessage {
    DevicesList,
    DeviceView,
}

#[derive(Default)]
struct MeshChat {
    view: View,
    device_list_view: DeviceListView,
    device_view: DeviceView,
    errors: Vec<(String, String)>,
}

/// These are the messages that MeshChat responds to
#[derive(Debug, Clone)]
pub enum Message {
    Navigation(NavigationMessage),
    WindowEvent(Event),
    Discovery(DiscoveryEvent),
    Device(DeviceViewMessage),
    Exit,
    NewConfig(Config),
    SaveConfig(Config),
    AppError(String, String),
    None,
}

fn main() -> iced::Result {
    iced::application(MeshChat::title, MeshChat::update, MeshChat::view)
        .subscription(MeshChat::subscription)
        .exit_on_close_request(false)
        .resizable(true)
        .run_with(MeshChat::new)
}

pub fn name_from_id(id: &BleId) -> String {
    match id {
        BleId::Name(name) => name.to_string(),
        BleId::MacAddress(mac) => mac.to_string(),
        BleId::NameAndMac(name, _) => name.to_string(),
    }
}

impl MeshChat {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::batch(vec![load_config()]))
    }

    fn title(&self) -> String {
        // Can enhance with the number of unread messages or something
        "MeshChat".to_string()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Navigation(navigation_message) => self.navigate(navigation_message),
            WindowEvent(event) => self.window_handler(event),
            Discovery(discovery_event) => self.device_list_view.update(discovery_event),
            Device(device_event) => self.device_view.update(device_event),
            Exit => window::get_latest().and_then(window::close),
            NewConfig(config) => {
                if let Some(name) = &config.device_name {
                    self.device_view.update(DeviceViewMessage::ConnectRequest(
                        name.clone(),
                        config.channel_number,
                    ))
                } else {
                    Task::none()
                }
            }
            AppError(summary, detail) => {
                eprintln!("{summary} {detail}");
                Task::none()
            }
            Message::None => Task::none(),
            Message::SaveConfig(config) => save_config(config),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut header = Row::new();
        let inner = match self.view {
            View::DeviceList => {
                header = self
                    .device_list_view
                    .header(header, self.device_view.connection_state());
                self.device_list_view
                    .view(self.device_view.connection_state())
            }
            View::Device => {
                header = self
                    .device_view
                    .header(header, self.device_view.connection_state());
                self.device_view.view()
            }
        };

        let mut outer = Column::new();

        outer = outer.push(header);
        outer = outer.push(self.errors());
        outer = outer.push(inner);

        outer.into()
    }

    fn errors(&self) -> Element<'_, Message> {
        let mut errors = Row::new().padding(10);

        // TODO a box with color and padding and a cancel button that removes this error
        // larger font for summary, detail can be unfolded
        for (summary, _details) in &self.errors {
            errors = errors.push(text(summary.clone()));
        }

        errors.into()
    }

    /// Subscribe to events from Discover and from Windows and from Devices (Radios)
    fn subscription(&self) -> Subscription<Message> {
        let subscriptions = vec![
            iced::event::listen().map(WindowEvent),
            Subscription::run(ble_discovery).map(Discovery),
            self.device_view.subscription().map(Device),
        ];

        Subscription::batch(subscriptions)
    }

    fn navigate(&mut self, navigation_message: NavigationMessage) -> Task<Message> {
        match navigation_message {
            NavigationMessage::DevicesList => {
                if self.view == View::Device {
                    self.view = View::DeviceList;
                }
            }
            NavigationMessage::DeviceView => {
                self.view = View::Device;
            }
        }
        Task::none()
    }

    fn window_handler(&mut self, event: Event) -> Task<Message> {
        if let Event::Window(window::Event::CloseRequested) = event {
            if let Connected(_id) = self.device_view.connection_state().clone() {
                // TODO send message to subscription to request we disconnect
                Task::none()
            } else {
                window::get_latest().and_then(window::close)
            }
        } else {
            Task::none()
        }
    }
}

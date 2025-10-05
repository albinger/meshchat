use crate::device_subscription::SubscriberMessage::{Connect, Disconnect};
use crate::device_subscription::SubscriptionEvent::{
    ConnectedEvent, DevicePacket, DisconnectedEvent, Ready,
};
use crate::device_subscription::{SubscriberMessage, SubscriptionEvent};
use crate::device_view::ConnectionState::{Connected, Connecting, Disconnected, Disconnecting};
use crate::device_view::DeviceViewMessage::{DeviceConnect, DeviceDisconnect, SubscriptionMessage};
use crate::{device_subscription, Message, NavigationMessage};
use iced::futures::channel::mpsc::Sender;
use iced::futures::SinkExt;
use iced::widget::{button, container, text, Column};
use iced::{Element, Length, Task};
use iced_futures::Subscription;
use meshtastic::utils::stream::BleId;

#[derive(Clone)]
pub enum ConnectionState {
    Disconnected,
    Connecting(BleId),
    Connected(BleId),
    Disconnecting(BleId),
}

#[derive(Debug, Clone)]
pub enum DeviceViewMessage {
    DeviceConnect(BleId),
    DeviceDisconnect(BleId),
    SubscriptionMessage(SubscriptionEvent),
}

pub struct DeviceView {
    pub connection_state: ConnectionState,
    subscription_sender: Option<Sender<SubscriberMessage>>, // TODO Maybe combine with Disconnected state?
}

async fn request_connection(mut sender: Sender<SubscriberMessage>, id: BleId) {
    let _ = sender.send(Connect(id)).await;
}

async fn request_disconnection(mut sender: Sender<SubscriberMessage>) {
    let _ = sender.send(Disconnect).await;
}

async fn empty() {}

impl DeviceView {
    pub fn new() -> Self {
        Self {
            connection_state: Disconnected,
            subscription_sender: None,
        }
    }

    pub fn connection_state(&self) -> &ConnectionState {
        &self.connection_state
    }

    /// Return a true value to show we can show the device view, false for main to decide
    pub fn update(&mut self, device_event: DeviceViewMessage) -> Task<Message> {
        match device_event {
            DeviceConnect(id) => {
                self.connection_state = Connecting(id.clone()); // TODO make state change depend on message back from subscriptino
                let sender = self.subscription_sender.clone();
                Task::perform(request_connection(sender.unwrap(), id), |_| {
                    Message::Navigation(NavigationMessage::Connecting)
                })
            }
            DeviceDisconnect(id) => {
                self.connection_state = Disconnecting(id.clone()); // TODO make state change depend on message back from subscriptino
                // Send a message to the subscription to disconnect
                let sender = self.subscription_sender.clone();
                Task::perform(request_disconnection(sender.unwrap()), |_| {
                    Message::Navigation(NavigationMessage::Back)
                })
            }
            SubscriptionMessage(device_event) => match device_event {
                ConnectedEvent(id) => {
                    self.connection_state = Connected(id);
                    Task::none()
                }
                DisconnectedEvent(_) => {
                    self.connection_state = Disconnected;
                    Task::perform(empty(), |_| Message::Navigation(NavigationMessage::Back))
                }
                Ready(sender) => {
                    println!("Subscriber is ready");
                    self.subscription_sender = Some(sender);
                    Task::none()
                }
                DevicePacket(packet) => {
                    println!("Received packet: {:?}", packet);
                    Task::none()
                }
            },
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

    /// Create subscriptions for events coming from a connected hardware device (radio)
    pub fn subscription(&self) -> Subscription<DeviceViewMessage> {
        let subscriptions = vec![
            Subscription::run_with_id("device", device_subscription::subscribe())
                .map(SubscriptionMessage),
        ];

        Subscription::batch(subscriptions)
    }
}

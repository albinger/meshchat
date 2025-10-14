use crate::channel_view::ChannelViewMessage::MessageInput;
use crate::device_view::DeviceViewMessage::{ChannelMsg, SendMessage};
use crate::Message;
use iced::widget::scrollable::Scrollbar;
use iced::widget::{scrollable, text, text_input, Column, Row};
use iced::{Element, Fill, Task};
use meshtastic::protobufs::mesh_packet::PayloadVariant::Decoded;
use meshtastic::protobufs::{MeshPacket, PortNum};

#[derive(Debug, Clone)]
pub enum ChannelViewMessage {
    MessageInput(String),
}

pub struct ChannelView {
    pub(crate) channel_index: i32, // Channel number of the channel we are chatting on
    pub packets: Vec<MeshPacket>,
    pub message: String, // Message typed in so far
}

impl ChannelView {
    pub fn new(channel_index: i32) -> Self {
        Self {
            channel_index,
            packets: Vec::new(),
            message: String::new(),
        }
    }

    pub fn message_sent(&mut self) {
        // TODO Mark as sent in the UI, and clear the message entry
        // Until we have some kind of queue of messages being sent pending confirmation
        self.message = String::new();
    }

    pub fn push_packet(&mut self, mesh_packet: MeshPacket) {
        self.packets.push(mesh_packet);
    }

    pub fn num_packets(&self) -> usize {
        self.packets.len()
    }

    pub fn update(&mut self, channel_view_message: ChannelViewMessage) -> Task<Message> {
        match channel_view_message {
            MessageInput(s) => {
                self.message = s;
                Task::none()
            }
        }
    }

    // Make this a struct and move the message field in here
    pub fn view(&self) -> Element<'static, Message> {
        let mut channel_view = Column::new();

        for packet in &self.packets {
            if let Some(Decoded(data)) = &packet.payload_variant
                && data.emoji == 0
            // TODO handle emoji replies
            {
                match PortNum::try_from(data.portnum) {
                    Ok(PortNum::TextMessageApp) => {
                        let mut packet_row = Row::new();
                        packet_row = packet_row.push(
                            text(String::from_utf8(data.payload.clone()).unwrap())
                                .shaping(text::Shaping::Advanced),
                        );
                        channel_view = channel_view.push(packet_row);
                    }
                    Ok(PortNum::PositionApp) => println!("Position payload"),
                    Ok(PortNum::AlertApp) => println!("Alert payload"),
                    Ok(PortNum::TelemetryApp) => println!("Telemetry payload"),
                    Ok(PortNum::NeighborinfoApp) => println!("Neighbor Info payload"),
                    Ok(PortNum::NodeinfoApp) => println!("Node Info payload"),
                    _ => eprintln!("Unexpected payload type from radio: {}", data.portnum),
                }
            }
        }

        let channel_scroll = scrollable(channel_view)
            .direction({
                let scrollbar = Scrollbar::new().width(10.0);
                scrollable::Direction::Vertical(scrollbar)
            })
            .width(Fill)
            .height(Fill);

        // TODO set an icon,
        // TODO Add to messages in the channel for display, or wait for packet back from radio
        // as a confirmation? Maybe add as sending status?
        // Display it just above the text input until confirmed by arriving in channel?
        // for now only sent to the subscription
        // TODO add an id to the message, or get it back from the subscription to be
        // able to handle replies to it later. Get a timestamp and maybe sender id
        // when TextSent then add to the UI list of messages, interleaved with
        // those received using the timestamp
        let text_box = text_input("Message>", &self.message)
            .on_input(|s| Message::Device(ChannelMsg(MessageInput(s))))
            .on_submit(Message::Device(SendMessage(
                self.message.clone(),
                self.channel_index,
            )));
        let bottom_row = Row::new().push(text_box);

        Column::new().push(channel_scroll).push(bottom_row).into()
    }
}

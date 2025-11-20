use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Serialize, Deserialize)]
pub enum Payload {
    NewTextMessage(String),
    /// TextMessageReply(reply_to_id, reply text)
    TextMessageReply(u32, String),
    /// EmojiReply(reply_to_id, emoji_code string)
    EmojiReply(u32, String),
    Position(i32, i32),
    Ping(String), // Could add hw_model or similar if wanted
}

/// An entry in the Channel View that represents some type of message sent to either this user on
/// this device or to a channel this device can read. Can be any of [Payload] types.
#[derive(Clone, Serialize, Deserialize)]
pub struct ChannelViewEntry {
    from: u32,
    message_id: u32,
    rx_time: u64,
    payload: Payload,
    name: Option<String>,
    seen: bool,
    acked: bool,
    /// Map of emojis and for each emoji there is the string for it and a number of node ides
    /// who sent that emoji
    emoji_reply: HashMap<String, Vec<String>>,
}

impl ChannelViewEntry {
    /// Create a new [ChannelViewEntry] from the parameters provided. The received time will be set to
    /// the current time in EPOC as an u64
    pub fn new(
        message: Payload,
        from: u32,
        message_id: u32,
        name: Option<String>,
        seen: bool,
    ) -> Self {
        let rx_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|t| t.as_secs())
            .unwrap_or(0);

        ChannelViewEntry {
            payload: message,
            from,
            message_id,
            rx_time,
            name,
            seen,
            acked: false,
            emoji_reply: HashMap::new(),
        }
    }

    /// Return the node id that sent the message
    pub fn from(&self) -> u32 {
        self.from
    }

    /// Get a reference to the payload of this message
    pub fn payload(&self) -> &Payload {
        &self.payload
    }

    /// Return true if this message was sent from the specified node id
    pub fn source_node(&self, node_id: u32) -> bool {
        self.from == node_id
    }

    /// Return the message_id
    pub fn message_id(&self) -> u32 {
        self.message_id
    }

    /// Mark the Entry as acknowledgeMd
    pub fn ack(&mut self) {
        self.acked = true;
    }

    /// Add an emoji reply to this entry
    pub fn add_emoji(&mut self, emoji_string: String, emoji_source: String) {
        self.emoji_reply
            .entry(emoji_string)
            .and_modify(|sender_vec| sender_vec.push(emoji_source.clone()))
            .or_insert(vec![emoji_source]);
    }

    /// Return true if the radio has acknowledged this message
    pub fn acked(&self) -> bool {
        self.acked
    }

    /// Return the emoji reply to this message, if any.
    pub fn emojis(&self) -> &HashMap<String, Vec<String>> {
        &self.emoji_reply
    }

    /// Return the time this message was received/sent as u64 seconds in EPOCH time
    pub fn time(&self) -> u64 {
        self.rx_time
    }

    /// Return the optional name of the sender of this message
    pub fn name(&self) -> &Option<String> {
        &self.name
    }
}

impl PartialEq<Self> for ChannelViewEntry {
    fn eq(&self, other: &Self) -> bool {
        self.rx_time == other.rx_time
    }
}

impl PartialOrd<Self> for ChannelViewEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChannelViewEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rx_time.cmp(&other.rx_time)
    }
}

impl Eq for ChannelViewEntry {}

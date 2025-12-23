use emojis::Group;
use iced::{
    Element, Theme,
    widget::{button, container, grid, scrollable, text, text::Shaping::Advanced, tooltip},
};

/// Message type for EmojiPicker
#[derive(Debug, Clone)]
pub enum PickerMessage {
    GroupSelected(Group),
    EmojiSelected(String),
}

/// State for the EmojiPicker widget
#[derive(Debug, Clone)]
pub struct EmojiPicker {
    group: Group,
}

impl EmojiPicker {
    pub fn new() -> Self {
        Self {
            group: Group::SmileysAndEmotion,
        }
    }

    /// Set the current emoji group
    pub fn with_group(mut self, group: Group) -> Self {
        self.group = group;
        self
    }

    /// Update the picker state
    pub fn update(&mut self, message: PickerMessage) {
        match message {
            PickerMessage::GroupSelected(group) => {
                self.group = group;
            }
            PickerMessage::EmojiSelected(_) => {
                // Emoji selection is handled by parent through the on_select closure
            }
        }
    }

    /// Create the view for the emoji picker
    /// The on_select closure is called with the selected emoji string and should return your Message type
    pub fn view<'a, Message: 'a>(
        &self,
        on_select: impl Fn(String) -> Message + 'a,
    ) -> Element<'a, Message>
    where
        Message: Clone,
    {
        const SPACING: u32 = 3;

        // For simplicity in the inline usage, we won't support group switching
        // The picker will always show the same group (the one it was initialized with)
        // If you need group switching, store the EmojiPicker in your app state and handle PickerMessage

        let emojis = self.group.emojis().collect::<Vec<_>>();
        let mut items = vec![];

        for emoji in emojis {
            items.push(Element::from(
                tooltip(
                    button(text(emoji.as_str()).center().shaping(Advanced).size(30))
                        .on_press(on_select(emoji.to_string())),
                    text(emoji.name()),
                    tooltip::Position::default(),
                )
                .style(|theme: &Theme| container::Style {
                    background: Some(theme.palette().background.into()),
                    ..Default::default()
                }),
            ));
        }

        let grid = grid(items).fluid(50).spacing(SPACING);

        scrollable(grid).spacing(SPACING).into()
    }
}

impl Default for EmojiPicker {
    fn default() -> Self {
        Self::new()
    }
}

use serde::de::{Deserialize, Deserializer, Error as DeError};
use serde::ser::{Serialize, Serializer};

use crate::internal::prelude::*;
use crate::json::from_value;
use crate::model::channel::ReactionType;

enum_number! {
    /// The type of a component
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

    #[non_exhaustive]
    pub enum ComponentType {
        ActionRow = 1,
        Button = 2,
        StringSelect = 3,
        InputText = 4,
        UserSelect = 5,
        RoleSelect = 6,
        MentionableSelect = 7,
        ChannelSelect = 8,
        _ => Unknown(u8),
    }
}

/// An action row.
#[derive(Clone, Debug)]
pub struct ActionRow {
    /// The type of component this ActionRow is.
    pub kind: ComponentType,
    /// The components of this ActionRow.
    pub components: Vec<ActionRowComponent>,
}

// A component which can be inside of an [`ActionRow`].
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ActionRowComponent {
    Button(Button),
    SelectMenu(SelectMenu),
    InputText(InputText),
}

impl From<Button> for ActionRowComponent {
    fn from(component: Button) -> Self {
        ActionRowComponent::Button(component)
    }
}

impl From<SelectMenu> for ActionRowComponent {
    fn from(component: SelectMenu) -> Self {
        ActionRowComponent::SelectMenu(component)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]

pub enum ButtonKind {
    Link { url: String },
    NonLink { custom_id: String, style: ButtonStyle },
}

/// A button component.
///
/// [Discord docs](https://discord.com/developers/docs/interactions/message-components#button-object-button-structure).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Button {
    /// The component type, it will always be [`ComponentType::Button`].
    pub kind: ComponentType,
    /// The button kind and style.
    pub data: ButtonKind,
    /// The text which appears on the button.
    pub label: String,
    /// The emoji of this button, if there is one.
    pub emoji: Option<ReactionType>,
    /// Whether the button is disabled.
    pub disabled: bool,
}

enum_number! {
    /// The style of a button.
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

    #[non_exhaustive]
    pub enum ButtonStyle {
        Primary = 1,
        Secondary = 2,
        Success = 3,
        Danger = 4,
        // No Link, because we represent Link using enum variants
        _ => Unknown(u8),
    }
}

/// A select menu component.
#[derive(Clone, Debug)]
pub struct SelectMenu {
    /// The component type, which may either be [`ComponentType::StringSelect`],
    /// [`ComponentType::UserSelect`], [`ComponentType::RoleSelect`],
    /// [`ComponentType::MentionableSelect`], or [`ComponentType::ChannelSelect`].
    pub kind: ComponentType,
    /// The placeholder shown when nothing is selected.
    pub placeholder: Option<String>,
    /// An identifier defined by the developer for the select menu.
    pub custom_id: Option<String>,
    /// The minimum number of selections allowed.
    pub min_values: Option<u64>,
    /// The maximum number of selections allowed.
    pub max_values: Option<u64>,
    /// The options of this select menu.
    pub options: Vec<SelectMenuOption>,
    /// The result location for modals
    pub values: Vec<String>,
}

/// A select menu component options.
#[derive(Clone, Debug)]
pub struct SelectMenuOption {
    /// The text displayed on this option.
    pub label: String,
    /// The value to be sent for this option.
    pub value: String,
    /// The description shown for this option.
    pub description: Option<String>,
    /// The emoji displayed on this option.
    pub emoji: Option<ReactionType>,
    /// Render this option as the default selection.
    pub default: bool,
}

/// An input text component for modal interactions
#[derive(Clone, Debug)]
pub struct InputText {
    /// The component type, it will always be [`ComponentType::InputText`].
    pub kind: ComponentType,
    /// An identifier defined by the developer for the select menu.
    pub custom_id: String,
    /// The input from the user
    pub value: String,
}

enum_number! {
    /// The style of the input text
    ///
    /// [Discord docs](https://discord.com/developers/docs/interactions/message-components#text-inputs-text-input-styles).
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]

    #[non_exhaustive]
    pub enum InputTextStyle {
        Short = 1,
        Paragraph = 2,
        _ => Unknown(u8),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::{assert_json, json};

    #[test]
    fn test_button_serde() {
        let mut button = Button {
            kind: ComponentType::Button,
            data: ButtonKind::NonLink {
                custom_id: "hello".into(),
                style: ButtonStyle::Danger,
            },
            label: "a".into(),
            emoji: None,
            disabled: false,
        };
        assert_json(
            &button,
            json!({"type": 2, "style": 4, "custom_id": "hello", "label": "a", "disabled": false}),
        );

        button.data = ButtonKind::Link {
            url: "https://google.com".into(),
        };
        assert_json(
            &button,
            json!({"type": 2, "style": 5, "url": "https://google.com", "label": "a", "disabled": false}),
        );
    }
}

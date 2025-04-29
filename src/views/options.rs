use iced::{
    Element, Font, Task,
    font::Weight,
    widget::{button, column, horizontal_space, row, text, text_input},
};

use crate::{Class, Message, SharedState};

#[derive(Debug, Clone)]
pub struct OptionsState {
    class_input_label: String,
    class_input_repeats: usize,
}

impl Default for OptionsState {
    fn default() -> Self {
        Self {
            class_input_label: String::new(),
            class_input_repeats: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OptionsMessage {
    /// (class label, repeats)
    AddClass((String, usize)),
    DeleteClass(usize),
    InputChange((InputKind, String)),
}

#[derive(Debug, Clone)]
pub enum InputKind {
    ClassLabel,
    ClassRepeats,
    PromptPrefill,
}

impl Into<Message> for OptionsMessage {
    fn into(self) -> Message {
        Message::Options(self)
    }
}

pub fn view<'a>(shared: &'a SharedState, local: &'a OptionsState) -> Element<'a, Message> {
    column![
        row![
            text("Classes").font(Font {
                weight: Weight::Bold,
                ..Default::default()
            }),
            horizontal_space(),
            button("Label").on_press_maybe(if shared.classes.len() != 0 {
                Some(Message::GoLabel)
            } else {
                None
            })
        ],
        row![
            text_input("Class Name", &local.class_input_label)
                .on_input(|x| OptionsMessage::InputChange((InputKind::ClassLabel, x)).into()),
            text_input("Repeats", &local.class_input_repeats.to_string())
                .on_input(|x| OptionsMessage::InputChange((InputKind::ClassRepeats, x)).into()),
            button("Add").on_press_maybe(
                if local.class_input_label.len() != 0 && local.class_input_repeats != 0 {
                    Some(
                        OptionsMessage::AddClass((
                            local.class_input_label.clone(),
                            local.class_input_repeats,
                        ))
                        .into(),
                    )
                } else {
                    None
                }
            )
        ]
        .spacing(10),
        if shared.classes.len() != 0 {
            column(shared.classes.iter().enumerate().map(|(index, set)| {
                row![
                    text(format!(
                        "{} ({} repeats)",
                        &set.label,
                        set.repeats.to_string()
                    )),
                    button("Remove").on_press(OptionsMessage::DeleteClass(index).into())
                ]
                .spacing(10)
                .into()
            }))
            .spacing(5)
            .into()
        } else {
            Element::from(text("None"))
        },
        text("Pre-filled Prompt").font(Font {
            weight: Weight::Bold,
            ..Default::default()
        }),
        text_input(
            "Prompt",
            shared.prompt_prefill.as_ref().unwrap_or(&String::new())
        )
        .on_input(|x| OptionsMessage::InputChange((InputKind::PromptPrefill, x)).into())
    ]
    .spacing(10)
    .into()
}

pub fn update(
    state: &mut SharedState,
    local: &mut OptionsState,
    message: OptionsMessage,
) -> Task<Message> {
    match message {
        OptionsMessage::InputChange((kind, value)) => match kind {
            InputKind::ClassLabel => local.class_input_label = value,
            InputKind::ClassRepeats => {
                if let Ok(value) = usize::from_str_radix(&value, 10) {
                    local.class_input_repeats = value
                }
            }
            InputKind::PromptPrefill => {
                if value.len() != 0 {
                    state.prompt_prefill = Some(value);
                } else {
                    state.prompt_prefill = None;
                }
            }
        },

        OptionsMessage::AddClass((label, repeats)) => {
            state.classes.push(Class { label, repeats });
        }

        OptionsMessage::DeleteClass(index) => {
            state.classes.remove(index);
        }
    }

    Task::none()
}

use std::path::PathBuf;

use iced::{
    Element, Task,
    widget::{button, column, horizontal_space, row, text, text_input},
};
use rfd::FileDialog;

use crate::Message;

#[derive(Debug, Clone)]
pub struct SetupState {
    pub input_dir: Option<PathBuf>,
    pub output_dir: Option<PathBuf>,
    pub trash_dir: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum DirectoryKind {
    Input,
    Output,
    Trash,
}

#[derive(Debug, Clone)]
pub enum SetupMessage {
    SetPath(DirectoryKind, PathBuf),
    OpenChooser(DirectoryKind),
}

impl Into<Message> for SetupMessage {
    fn into(self) -> Message {
        Message::Setup(self)
    }
}

fn opt_buf_to_string(buf: Option<PathBuf>) -> String {
    buf.map_or("".to_owned(), |x| x.to_string_lossy().to_string())
}

pub fn view(state: &SetupState) -> Element<Message> {
    column![
        column![
            text("Input Directory"),
            row![
                text_input("Unset", &opt_buf_to_string(state.input_dir.clone()),),
                button("Select").on_press(SetupMessage::OpenChooser(DirectoryKind::Input).into())
            ]
            .spacing(10),
        ],
        column![
            text("Output Directory"),
            row![
                text_input("Unset", &opt_buf_to_string(state.output_dir.clone()),),
                button("Select").on_press(SetupMessage::OpenChooser(DirectoryKind::Output).into())
            ]
            .spacing(10),
        ],
        column![
            text("Trash Directory"),
            row![
                text_input("Unset", &opt_buf_to_string(state.trash_dir.clone()),),
                button("Select").on_press(SetupMessage::OpenChooser(DirectoryKind::Trash).into())
            ]
            .spacing(10),
        ],
        row![
            horizontal_space(),
            button("Begin").on_press_maybe(
                if state.output_dir.is_some() && state.input_dir.is_some() {
                    Some(Message::SetupDone(state.clone()))
                } else {
                    None
                }
            )
        ]
    ]
    .spacing(10)
    .into()
}

pub fn update(state: &mut SetupState, message: SetupMessage) -> Task<Message> {
    match message {
        SetupMessage::OpenChooser(directory_kind) => {
            return Task::perform(
                async {
                    FileDialog::new()
                        .set_can_create_directories(true)
                        .pick_folder()
                },
                move |path| {
                    if let Some(path) = path {
                        SetupMessage::SetPath(directory_kind.clone(), path).into()
                    } else {
                        Message::NoOp
                    }
                },
            );
        }

        SetupMessage::SetPath(kind, path) => match kind {
            DirectoryKind::Input => {
                state.input_dir = Some(path);
            }
            DirectoryKind::Output => {
                state.output_dir = Some(path);
            }
            DirectoryKind::Trash => {
                state.trash_dir = Some(path);
            }
        },
    }

    Task::none()
}

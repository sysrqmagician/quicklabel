use std::path::PathBuf;

use iced::{
    Element, Font, Task,
    font::Weight,
    widget::{button, column, container, horizontal_space, row, text},
};
use views::{
    options::{OptionsMessage, OptionsState},
    setup::{SetupMessage, SetupState},
};

mod views;

#[derive(Debug, Clone)]
struct SharedState {
    /// Dumping directory with images
    input_dir: PathBuf,
    /// Root directory of dreambooth-style dataset
    output_dir: PathBuf,
    /// If None, trashed images will be left in input_dir
    trash_dir: Option<PathBuf>,

    classes: Vec<Class>,
    prompt_prefill: Option<String>,
}

impl Into<SharedState> for SetupState {
    fn into(self) -> SharedState {
        SharedState {
            input_dir: self
                .input_dir
                .expect("Unreachable due to on_press_maybe condition"),
            output_dir: self
                .output_dir
                .expect("Unreachable due to on_press_maybe condition"),
            trash_dir: self.trash_dir,
            classes: Vec::new(),
            prompt_prefill: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    NoOp,
    ResetState,
    Setup(SetupMessage),
    SetupDone(SetupState),
    Options(OptionsMessage),
    FatalError(String),
}

#[derive(Debug, Clone)]
struct Class {
    label: String,
    repeats: usize,
}

#[derive(Debug, Clone)]
enum View {
    Setup(SetupState),
    Options(SharedState, OptionsState),
    // Labeling(State),
    FatalError(String),
}

impl Default for View {
    fn default() -> Self {
        Self::Setup(SetupState {
            input_dir: None,
            output_dir: None,
            trash_dir: None,
        })
    }
}

impl View {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NoOp => {}

            Message::ResetState => {
                *self = View::default();
            }

            Message::FatalError(error_description) => {
                *self = View::FatalError(error_description);
            }

            Message::Setup(message) => {
                if let View::Setup(setup) = self {
                    return views::setup::update(setup, message);
                } else {
                    panic!("Setup Message sent by other view?! -- {:#?}", self);
                }
            }

            Message::SetupDone(setup) => {
                *self = View::Options(setup.into(), OptionsState::default());
            }

            Message::Options(message) => {
                if let View::Options(shared, local) = self {
                    return views::options::update(shared, local, message);
                } else {
                    panic!("Options Message sent by other view?! -- {:#?}", self);
                }
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        column![
            row![
                text(self.title()).font(Font {
                    weight: Weight::Light,
                    ..Default::default()
                }),
                horizontal_space(),
                text(format!(
                    "sysrqmagician/quicklabel v{}",
                    env!("CARGO_PKG_VERSION"),
                ))
            ],
            container(match self {
                View::Setup(setup) => views::setup::view(setup),
                View::Options(shared, local) => views::options::view(shared, local),
                View::FatalError(message) => column![
                    text(message),
                    button("Restart").on_press(Message::ResetState)
                ]
                .into(),
            })
            .padding(10)
        ]
        .padding(10)
        .into()
    }

    fn title(&self) -> &str {
        match self {
            View::Setup(..) => "Directories",
            View::Options(..) => "Options",
            View::FatalError(..) => "Fatal Error",
        }
    }
}

#[tokio::main]
async fn main() {
    iced::application("QuickLabel", View::update, View::view)
        .run()
        .expect("Failed to run GUI");
}

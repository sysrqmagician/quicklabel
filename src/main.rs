use std::path::PathBuf;

use iced::{
    Element, Font, Task,
    font::Weight,
    widget::{button, column, container, horizontal_space, row, text},
};
use views::{
    labeling::{LabelingMessage, LabelingState},
    options::{OptionsMessage, OptionsState},
    setup::{SetupMessage, SetupState},
};

mod views;

#[derive(Debug, Clone, Default)]
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
    GoOptions,
    GoLabel,
    Labeling(LabelingMessage),
    ShowText(String, String),
    FatalError(String),
}

#[derive(Debug, Clone)]
struct Class {
    label: String,
    repeats: usize,
}

impl Class {
    fn path(&self, mut output_root: PathBuf) -> PathBuf {
        output_root.push(format!("{}_{}", self.repeats, self.label));

        output_root
    }
}

#[derive(Debug, Clone)]
enum View {
    Setup(SetupState),
    Options(SharedState, OptionsState),
    Labeling(SharedState, LabelingState),
    FatalError(Option<String>, String),
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
                *self = View::FatalError(None, error_description);
            }

            Message::ShowText(title, body) => {
                *self = View::FatalError(Some(title), body);
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

            Message::GoOptions => {
                if let View::Labeling(shared, ..) = self {
                    *self = View::Options(std::mem::take(shared), OptionsState::default());
                } else {
                    panic!("GoOptions from other view?! -- {:#?}", self);
                }
            }

            Message::GoLabel => {
                if let View::Options(shared, ..) = self {
                    // Ensure all directories exist before proceeding
                    for class in &shared.classes {
                        let class_dir = class.path(shared.output_dir.clone());
                        if let Err(e) = std::fs::create_dir_all(&class_dir) {
                            return Task::done(Message::FatalError(format!(
                                "Unable to create directory {class_dir:#?}: {e}"
                            )));
                        }
                    }

                    *self = View::Labeling(std::mem::take(shared), LabelingState::default());

                    return Task::done(LabelingMessage::Index.into());
                } else {
                    panic!("GoLabel from other view?! -- {:#?}", self);
                }
            }

            Message::Labeling(message) => {
                if let View::Labeling(shared, local) = self {
                    return views::labeling::update(shared, local, message);
                } else {
                    panic!("Labeling Message sent by other view?! -- {:#?}", self);
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
                View::Labeling(shared, local) => views::labeling::view(shared, local),
                View::FatalError(.., message) => column![
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
            View::FatalError(title, ..) => {
                if let Some(title) = title {
                    title
                } else {
                    "Fatal Error"
                }
            }
            View::Labeling(..) => "Label",
        }
    }
}

#[tokio::main]
async fn main() {
    println!(
        "quicklabel v{}; https://github.com/sysrqmagician/quicklabel\nCopyright (c) 2025, sysrqmagician <sysrqmagician@proton.me>\n",
        env!("CARGO_PKG_VERSION")
    );
    println!(include_str!("../LICENSE"));

    iced::application("quicklabel", View::update, View::view)
        .run()
        .expect("Failed to run GUI");
}

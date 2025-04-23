use std::path::PathBuf;

use iced::{
    Element, Font, Length, Task,
    font::Weight,
    widget::{column, container, horizontal_space, row, text},
};
use views::setup::{SetupMessage, SetupState};

mod views;

#[derive(Debug, Clone)]
struct State {
    /// Dumping directory with images
    input_dir: PathBuf,
    /// Root directory of dreambooth-style dataset
    output_dir: PathBuf,
    /// If None, trashed images will be left in input_dir
    trash_dir: Option<PathBuf>,

    /// (repeats, subject class)
    datasets: Vec<Dataset>,

    label_prefill: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    NoOp,
    Setup(SetupMessage),
    SetupDone(SetupState),
}

#[derive(Debug, Clone)]
struct Dataset {
    class: String,
    repeats: usize,
    path: PathBuf,
}

#[derive(Debug, Clone)]
enum View {
    Setup(SetupState),
    // Options(State),
    // Labeling(State),
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
            Message::NoOp => Task::none(),
            Message::Setup(setup_message) => {
                if let View::Setup(setup) = self {
                    views::setup::update(setup, setup_message)
                } else {
                    panic!("Setup Message sent by other view?! -- {:#?}", self);
                }
            }
            Message::SetupDone(setup) => {
                todo!()
            }
        }
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
            })
            .padding(10)
        ]
        .padding(10)
        .into()
    }

    fn title(&self) -> &str {
        match self {
            View::Setup(_) => "Setup",
        }
    }
}

#[tokio::main]
async fn main() {
    iced::application("QuickLabel", View::update, View::view)
        .run()
        .expect("Failed to run GUI");
}

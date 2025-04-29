use std::path::PathBuf;

use iced::{
    Alignment::Center,
    Element, Length, Task,
    widget::{button, column, container, horizontal_space, image, row, text, text_input},
};

use crate::{Message, SharedState};

pub const IMAGE_EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "webp"];

#[derive(Debug, Clone, Default)]
pub struct LabelingState {
    images: Vec<PathBuf>,
    images_cursor: usize,
    current_image: Option<PathBuf>,
    input_prompt: String,
}

#[derive(Debug, Clone)]
pub enum LabelingMessage {
    SetPrompt(String),
    /// class index
    SubmitLabel(usize),
    Index,
    FinishIndexing(Vec<PathBuf>),
    NextImage,
    NoImagesLeft,
    TrashCurrent,
}

impl Into<Message> for LabelingMessage {
    fn into(self) -> Message {
        Message::Labeling(self)
    }
}

pub fn update(
    shared: &mut SharedState,
    local: &mut LabelingState,
    message: LabelingMessage,
) -> Task<Message> {
    match message {
        LabelingMessage::Index => {
            let input_dir = shared.input_dir.clone();
            return Task::perform(
                async move {
                    let read_dir = match std::fs::read_dir(&input_dir) {
                        Ok(read_dir) => read_dir,
                        Err(e) => {
                            return Message::FatalError(format!(
                                "Failed to read input directory: {e}"
                            ));
                        }
                    };

                    let mut images = Vec::new();
                    for entry in read_dir {
                        match entry {
                            Ok(entry) => {
                                let path = entry.path();
                                if path.is_file()
                                    && IMAGE_EXTENSIONS.contains(
                                        &path
                                            .extension()
                                            .unwrap_or_default()
                                            .display()
                                            .to_string()
                                            .to_lowercase()
                                            .as_str(),
                                    )
                                {
                                    images.push(path);
                                }
                            }
                            Err(e) => {
                                return Message::FatalError(format!("Failed to read entry: {e}"));
                            }
                        }
                    }
                    return LabelingMessage::FinishIndexing(images).into();
                },
                |out| out,
            );
        }

        LabelingMessage::FinishIndexing(images) => {
            if images.is_empty() {
                return Task::done(Message::FatalError("No images found".to_string()));
            }

            local.images = images;
            local.current_image = Some(
                local
                    .images
                    .first()
                    .expect("No first image despite length check?!")
                    .clone(),
            );
            local.images_cursor = 0;
        }

        LabelingMessage::NextImage => {
            local.images_cursor += 1;

            if let Some(path) = local.images.get(local.images_cursor) {
                local.current_image = Some(path.clone());
            } else {
                return Task::done(LabelingMessage::NoImagesLeft.into());
            }
        }

        LabelingMessage::SetPrompt(value) => {
            local.input_prompt = value;
        }

        LabelingMessage::NoImagesLeft => {
            return Task::done(Message::ShowText(
                "Done!".into(),
                "No more images to label.".into(),
            ));
        }

        LabelingMessage::TrashCurrent => {
            let current_image = local
                .current_image
                .clone()
                .expect("Submitting without image?!");

            if let Some(mut trash_destination) = shared.trash_dir.clone() {
                trash_destination.push(&current_image.file_name().unwrap());

                return Task::perform(
                    async move {
                        if let Err(e) = std::fs::copy(&current_image, &trash_destination) {
                            return Message::FatalError(format!(
                                "Failed to move {current_image:#?} to {trash_destination:#?}: {e}"
                            ));
                        }

                        if let Err(e) = std::fs::remove_file(&current_image) {
                            return Message::FatalError(format!(
                                "Failed to delete {current_image:#?}: {e}"
                            ));
                        }

                        LabelingMessage::NextImage.into()
                    },
                    |out| out,
                );
            } else {
                // If there isn't a trash dir, have trashed images remain in input dir
                return Task::done(LabelingMessage::NextImage.into());
            }
        }
        LabelingMessage::SubmitLabel(class_index) => {
            let current_image = local
                .current_image
                .clone()
                .expect("Submitting without image?!");

            let mut destination_path = shared.classes[class_index].path(shared.output_dir.clone());
            destination_path.push(&current_image.file_name().unwrap());

            let prompt = std::mem::take(&mut local.input_prompt);

            return Task::perform(
                async move {
                    if let Err(e) = std::fs::copy(&current_image, &destination_path) {
                        return Message::FatalError(format!(
                            "Failed to copy image from {current_image:#?} to {destination_path:#?}: {e}"
                        ));
                    }

                    if let Err(e) = std::fs::remove_file(&current_image) {
                        return Message::FatalError(format!(
                            "Failed to remove image {current_image:#?}: {e}"
                        ));
                    }

                    destination_path.set_extension("txt");
                    if let Err(e) = std::fs::write(&destination_path, prompt) {
                        return Message::FatalError(format!(
                            "Failed to write prompt to {destination_path:#?}: {e}"
                        ));
                    }

                    LabelingMessage::NextImage.into()
                },
                |out| out,
            );
        }
    }
    Task::none()
}

pub fn view<'a>(shared: &'a SharedState, local: &'a LabelingState) -> Element<'a, Message> {
    column![
        row![
            horizontal_space(),
            button("Options").on_press(Message::GoOptions)
        ],
        text_input("Prompt", &local.input_prompt)
            .on_input(|input| LabelingMessage::SetPrompt(input).into()),
        row(shared.classes.iter().enumerate().map(|(index, class)| {
            button(class.label.as_str())
                .on_press(LabelingMessage::SubmitLabel(index).into())
                .into()
        }))
        .spacing(5),
        button("Trash").on_press(LabelingMessage::TrashCurrent.into()),
        container(if let Some(path) = &local.current_image {
            Element::from(image(path))
        } else {
            text("Loading...").into()
        })
        .width(Length::Fill)
        .align_x(Center),
    ]
    .spacing(5)
    .into()
}

# quicklabel

A fast and efficient image labeling tool for creating Text-To-Image finetuning datasets, as taken by the kohya_ss training scripts.

## Overview

quicklabel is designed for efficiently creating labeled image datasets in a dreambooth-style format. It simplifies the process of organizing your input images into class directories with accompanying text prompts.

## Features

- **Simple Workflow**: Three-step process for quick dataset organization
- **Class Management**: Create custom classes with configurable repeat parameters
- **Prompt Templates**: Pre-fill prompts to speed up repetitive labeling
- **Trash Option**: Easily discard unwanted images

## Installation
### From Cargo

```bash
cargo install quicklabel
```

### From Releases

Visit the [Releases page](https://github.com/sysrqmagician/quicklabel/releases) and download the latest executable for your platform.

## Usage

### 1. Directory Setup

First, configure your directories:
- **Input Directory**: Folder containing unlabeled images
- **Output Directory**: Root folder where class directories will be created
- **Trash Directory** (Optional): Where discarded images are moved. If not specified, they will remain in the input directory when discarded.

### 2. Class Configuration

Define your classes:
- Enter a class name and number of repeats
- The output folders will be created as `{repeats}_{class_name}`
- Optionally configure a prompt template that will be pre-filled during labeling

### 3. Image Labeling

Process your images:
- View each image and enter a prompt (or use the pre-filled template)
- Select a class by clicking its button
- Images are moved to the appropriate class folder with matching text files
- Use the "Trash" button to discard unwanted images


## License

Copyright © 2025, sysrqmagician <sysrqmagician@proton.me>

Licensed under the terms included in the [LICENSE](LICENSE) file.

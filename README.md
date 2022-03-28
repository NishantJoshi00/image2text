# Image 2 Text Application

This is a GUI + CLI application that performs OCR and provides a easy to use interface for both gui and terminal users.


## Requirements

| Libraries |   Installation     |
|-----------|--------------------|
| Rust      | [Click Here](https://www.rust-lang.org/tools/install) |
| Tesseract | [Click Here](https://tesseract-ocr.github.io/tessdoc/Installation.html) |
| Leptonica | `fedora: leptonica-devel, leponica` |
|           | `ubuntu: libleptonica-dev`          |
---
   
## Usage

- ### Installation
    - ```bash
        $ cargo build release
        ```
    - ```bash
        $ cargo install --path .
        ```
- ### Application
    - CLI
    ```bash
        $ img2tex <image-path>
    ```
    - GUI - The executable can also be converted into an application using a .desktop file.
    ```bash
        $ img2tex
    ```
    



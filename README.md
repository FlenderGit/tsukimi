# Tsukimi

![logo](ressources/logo.png )

## Presentation
Tsukimi is a service to help translate VN into multiple languages. This application is CLI-based andd allow users to create and manage translation projects, upload and download translation files, and collaborate with other translators.

This project is composed of:
* `tsukimi-cli`: the main CLI application
* `tsukimi-api`: the cdn to serve the translation files and visual novels engine libraries
* `tsukimi-web`: a web application to manage translation projects and collaborate with other translators
* `tsukimi-engines`: a collection of engines (as `.wasm`) to manage and create translation projects

## How to contribute
### Requirements
* Rust 1.80.0 or later

### Create a new engine

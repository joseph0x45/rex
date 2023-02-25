# Rex CLI
[![forthebadge made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/)

[![Build](https://github.com/TheWisePigeon/rex/actions/workflows/build.yml/badge.svg)](https://github.com/TheWisePigeon/rex/actions/workflows/build.yml)
## What is Rex?
Rex is a CLI that will help you scaffold your next Express project, using either TypeScript or JavaScript. But Rex can be much more.

The structure of the templates are one that I follow for all my projects. I based it on `Microservices architecture`. Sticking to that
structure improved my productivity and the readability, maintainability and scalability of my projects. That's why I decided to make a CLI that will help me
bootstrap such projects as quickly as possible. If you intend to follow the same structure as me, then Rex becomes kind of a Framework.

That's why I decided to call it a **Pseudo Framework**

## Usage
### Installation
⚠️Rex is available only on windows for now.

To install Rex on your system, head over to [this link](https://github.com/TheWisePigeon/rex/releases/tag/pre-release) and download the `RexCLI.exe`

Once the installation is complete, you will need to add Rex's folder to your path. Check in the `Program Files` folder, you will find a Rex folder, copy it's path and add it to your Path environment variable.

You should now be ready to use Rex. The usage is quite simple.

The syntax is `rex init {template} {project_name}` 
```bash
rex init js crudAPI
rex init ts hello_rex
```

Only "js" et "ts" templates are available for now. I intend to make users able to create their own templates in the future.

### Folder Structure
A rex scaffolded project structure looks like this

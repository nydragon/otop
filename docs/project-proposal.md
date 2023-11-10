# OTOP - system resource and process manager

- [Introduction](#introduction)
- [Motivation](#motivation)
- [Objectives](#objectives)
  - [Minimal](#minimal)
  - [Baseline](#baseline)
  - [Enhanced](#enhanced)
- [Development Practices \& Technologies](#development-practices--technologies)
  - [Web-App](#web-app)
  - [Server](#server)
  - [CI/CD](#cicd)
- [Roles](#roles)
- [Organisation](#organisation)
  - [Timeline](#timeline)
  - [Tools](#tools)
- [Design](#design)

## Introduction

In the realm of system administration, htop has long been a staple for monitoring and managing processes. Its interactive interface and comprehensive information have made it an indispensable tool for system administrators and users alike. However, as technology evolves and user expectations shift, it is time to reimagine htop for the modern era.

We propose a reinvention of htop, embracing a modern design and leveraging the power of Rust, a programming language renowned for its performance, safety, and concurrency features. Our goal is to create a process viewer that not only retains the core functionality of htop but also enhances it with a user-friendly interface, improved performance, and cross-platform compatibility.


## Motivation

The motivation to reinvent htop stems from a desire to modernize a widely used tool and enhance its capabilities for the ever-evolving technological landscape. htop has served as a cornerstone for system administrators and users, providing valuable insights into system processes and resource utilization. However, as technology advances and user expectations grow, the need for a more modern, intuitive, and performant process viewer becomes increasingly apparent.

## Objectives

Our reimagined htop will feature several key enhancements, including:

1. A modern and intuitive user interface: We will revamp the interface to make it more visually appealing and easier to navigate. This includes using modern design principles, improving color contrast, and providing clear visual cues for important information.

2. Enhanced performance and resource efficiency: By leveraging Rust's memory management and concurrency features, we can significantly improve htop's performance and reduce its resource consumption. This will make it more responsive and efficient, even on resource-constrained systems.

### Minimal

- **Web-based Desktop-format Application**
- **System Usage**
- **Process Usage**
- **In-browser terminal**

### Baseline

- Web-based Desktop-format Application
- System Usage
- Process Usage
- In-browser terminal
- **Basic Process Management**
  - **Adjust niceness**
  - **Kill programs**

### Enhanced

- Web-based Desktop-format Application
- System Usage
- Process Usage
- **Improved Process Management**
  - **Display process dependency**
  - **Suspend processes**
  - **Adjust niceness**
  - **Kill programs**
- **Terminal interface with [Ratatui](https://github.com/ratatui-org/ratatui)**
  - **Interface customization (config file)**

## Development Practices & Technologies

- [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
- [Git Flow](https://www.atlassian.com/git/tutorials/comparing-workflows/gitflow-workflow)

### Web-App

- [React](https://react.dev/) + [Typescript](https://www.typescriptlang.org/)
- [Tanstack Query](https://tanstack.com/query/v3/)
- [Xterm JS](http://xtermjs.org/)

### Server

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
  > axum is a web application framework that focuses on ergonomics and modularity.

### CI/CD

- GitHub Actions

## Roles

- **Paul**
  - Backend
  - Design
  - Web-Application
- **Nicolas**
  - Backend
  - CI/CD
  - Development Workflow
  -

## Organisation

### Timeline


### Tools

- GitHub Projects

## Design

Figma: https://www.figma.com/file/xEL1weYq616QSztBqTk34j/OTOP?type=design&node-id=0%3A1&mode=design&t=QW39uMIVnViSID3j-1
Github Palette: https://www.color-hex.com/color-palette/28549

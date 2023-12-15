<br/>
<p align="center">
  <a href="https://github.com/nydragon/otop">
    <img src="logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">OTOP</h3>

  <p align="center">
    A new vision of monitoring and managing processes
    <br/>
    <br/>
    <a href="https://github.com/nydragon/otop/issues">Report Bug</a>
    .
    <a href="https://github.com/nydragon/otop/issues">Request Feature</a>
  </p>
</p>

![Downloads](https://img.shields.io/github/downloads/nydragon/otop/total) ![Contributors](https://img.shields.io/github/contributors/nydragon/otop?color=dark-green) ![Issues](https://img.shields.io/github/issues/nydragon/otop) ![License](https://img.shields.io/github/license/nydragon/otop)

## Table Of Contents

- [Table Of Contents](#table-of-contents)
- [About The Project](#about-the-project)
- [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
    - [Frontend](#frontend)
    - [Backend](#backend)
    - [Frontend](#frontend-1)
- [Usage](#usage)
  - [Permament Installation](#permament-installation)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
  - [Creating A Pull Request](#creating-a-pull-request)
- [License](#license)
- [Authors](#authors)
- [Acknowledgements](#acknowledgements)

## About The Project

![Screen Shot](images/screenshot.png)

In the realm of system administration, htop has long been a staple for monitoring and managing processes. Its interactive interface and comprehensive information have made it an indispensable tool for system administrators and users alike. However, as technology evolves and user expectations shift, it is time to reimagine htop for the modern era.

We propose a reinvention of htop, embracing a modern design and leveraging the power of Rust, a programming language renowned for its performance, safety, and concurrency features. Our goal is to create a process viewer that not only retains the core functionality of htop but also enhances it with a user-friendly interface, improved performance, and cross-platform compatibility.

## Built With

The best tools was used for this project

* [React](https://fr.react.dev/)
* [Typescript](https://www.typescriptlang.org/)
* [Sass](https://sass-lang.com/)
* [Rust](https://www.rust-lang.org/fr)
* [Axum](https://github.com/tokio-rs/axum)

## Getting Started

### Prerequisites

As dependencies you will have to install the following software:

#### Frontend

- npm
- NodeJS

To easily manage your Node version it is recommended to use [`nvm`](https://github.com/nvm-sh/nvm) or any other kind of Node version manager. Please refer to the [.nvmrc](packages/frontend/.nvmrc)  file to know which Node version to download.

#### Backend

- Rust toolchain

You may find it on their [website](https://rustup.rs/).

#### Frontend

1. Clone the repo

```sh
git clone https://github.com/Nydragon/otop.git
```

2. Install NPM packages

```sh
npm install --prefix packages/frontend;
```

## Usage

To start the program on your local machine, you will need to start the backend, using the following command:
```sh
cargo run --manifest-path packages/backend;
```

And the frontend with the following:
```sh
npm run dev --prefix packages/frontend;
```

### Permament Installation

If you wish to install the program permanently, you can follow these steps:

```sh
cargo build --manifest-path packages/backend/Cargo.toml --release;
chmod +x packages/backend/target/release/otop;
cp packages/backend/target/release/otop /bin; # Or any other directory in $PATH

npm run build --prefix packages/frontend;
cp packages/frontend/dist -r /www; # Or any other directory
```

Then to launch the software execute the following commands:

```sh
otop 2> /dev/null &;
miniserve /www --index index.html; # replace `www` by the path in which reside your build frontend files.
```

Feel free to substitue miniserve by any other http server. And then finally connect to the public address of the hosting device at the port :8080;

## Roadmap

See the [open issues](https://github.com/nydragon/otop/issues) for a list of proposed features (and known issues).

## Contributing

Contributions are what make the open source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.
* If you have suggestions for adding or removing projects, feel free to [open an issue](https://github.com/nydragon/otop/issues/new) to discuss it, or directly create a pull request after you edit the *README.md* file with necessary changes.
* Please make sure you check your spelling and grammar.
* Create individual PR for each suggestion.

### Creating A Pull Request

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See [LICENSE](https://github.com/nydragon/otop/blob/main/LICENSE.md) for more information.

## Authors

* **Limeal (Paul G)** - *Comp Sci Student* - [Limeal (Paul G)](https://github.com/limeal) - *Frontend + Backend + ProcessLib*
* **Nydragon (Nicolas L)** - *Comp Sci Student* - [Nydragon (Nicolas L)](https://github.com/nydragon) - *Backend + ProcessLib*

## Acknowledgements

* [Limeal](https://github.com/limeal/)
* [Nydragon](https://github.com/nydragon/)

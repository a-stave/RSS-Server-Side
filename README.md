# Project Overview

Welcome to _RSS Server-side,_ an open-source web server for monitoring web resources. It provides a reusable backend service that makes it easy for client-side applications to track updates without having to implement their own polling, caching, and conditional request logic. Our goal is to make a server that embodies "politeness," using `ETag` and `Last-Modified` to minimize unnecessary requests while making it easier for developers to build apps that use popular protocols like RSS and Atom.

> **Note:** This project is being developed as part of PSU's CS Capstone, Fall-Winter cycle. Ownership of _RSS Server-side_ will revert to our sponsor at the conclusion of this cycle in March 2027!

<br />

## Features

### TODO

- Monitor URLs for changes, hashing pages where `ETag` or `Last-Modified` are unavailable.
- Support HTTP caching and conditional requests.
- Provide a browser-accessible API using Cross-Origin Resource Sharing (CORS).
- Support self-hosted and "hub" deployments.

<br />

## Installation

### Prerequisites

[Install Rust](https://rustup.rs/) if you haven't already.

### Clone the repo

```
git clone https://github.com/a-stave/RSS-Server-Side.git
cd RSS-Server-Side
```

[TODO: If we turn this into a crate or docker image add those here; `curl`, `wget`, and `brew` are also popular, but haven't looked into what that entails. See []]

### Building & Running (WIP)

```
// Build the project
cargo build

// Run the project
cargo run

// Run a release build
cargo run --release
```

<br />

### Build Configuration (WIP)

[TODO: Let user specify port, CORS, other stuff as well?]

| Environment<br />Variable | Description |
| :-----------------------: | ----------- |
|          `TODO`           | lorem ipsum |

## Contributing

In order to minimize issues, the team should adhere to the following:

1. Always start a task from the most up-to-date `main` branch.
2. Always develop in a feature branch. (Naming conventions below.)
3. Where desirable, make sure you write unit tests within your source module. If your code breaks an existing integration test, you may patch the test yourself, or disable it and create an issue or communicate over the team Discord.
4. Pull requests require at least **two** approving reviews and green tests before merging to `main`.

<br />

| Type          | Example              |
| ------------- | -------------------- |
| Feature       | `feature/caching`    |
| Bug fix       | `fix/name-collision` |
| Tests         | `test/log-to-CLI`    |
| Documentation | `doc/readme`         |

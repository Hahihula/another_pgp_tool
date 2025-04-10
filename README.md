# Another PGP Tool

A secure, cross-platform PGP encryption tool built with Rust and Dioxus.

## About

Another PGP Tool is a work-in-progress application designed to provide simple, secure PGP encryption capabilities across multiple platforms. Built with Rust and the Dioxus framework, this tool aims to make encryption accessible while maintaining the highest security standards.

## Features

- Cross-platform support (desktop, web, mobile(hopefullys))
- Simple, intuitive interface for PGP operations
- Built with memory-safe Rust language
- Open-source and fully auditable
- App has "no memory". It does not store any data. Even switching tabs erases all form fields.

## Security

Security is our top priority:

- **Fully Open Source**: All code is publicly available for review and audit
- **Verifiable Builds**: Binaries are built directly on GitHub from the repository code
- **No Hidden Code**: What you see in the repository is exactly what goes into the binary
- **Memory Safety**: Built with Rust, which provides memory safety guarantees without a garbage collector

This approach ensures that users can verify the security of the application themselves or rely on the community's ongoing code reviews.

## Installation

*Coming soon*

## Usage

*Coming soon*

## Development

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

## Contributing

This project is a work in progress, and contributions are welcome! You can help in several ways:

- **Issues**: Report bugs, suggest features, or ask questions through GitHub issues
- **Pull Requests**: Code contributions are highly appreciated
- **Code Reviews**: Help review the existing code to improve security and functionality

## Technology Stack

- **Rust**: A language empowering everyone to build reliable and efficient software
- **Dioxus**: A portable, performant, and ergonomic framework for building cross-platform user interfaces

## License

MIT

## Roadmap

- ✔ finish MVP
- fix platform web
- add tests
- add documentation
- add CI/CD
- multi recipient encryption
- UI redesign with better UX and mobile first design
- add mobile platform

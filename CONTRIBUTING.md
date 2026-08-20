# Contributing to LieCord

Thanks for contributing to LieCord.

Before opening an issue or pull request, please check the existing issues and discussions to make sure the problem or feature has not already been reported.

## Reporting Bugs

When reporting a bug, include enough information to reproduce it.

Please include:

* What happened
* Steps to reproduce the issue
* What you expected to happen
* What actually happened
* OS, browser, and relevant version information
* Screenshots or logs when useful

Open an issue here:

https://github.com/yourusername/liecord/issues

## Feature Requests

For feature requests, check existing issues and discussions first.

When opening an issue, use `[Feature Request]` in the title and explain:

* What you want to add or change
* Why the feature would be useful
* Any relevant examples or prior art

## Pull Requests

### 1. Fork the repository

Create a fork of LieCord and clone it locally.

### 2. Create a branch

Use a descriptive branch name:

```bash
git checkout -b feature/add-example
```

For bug fixes:

```bash
git checkout -b fix/fix-example
```

### 3. Make your changes

Keep changes focused and avoid unrelated modifications.

For new functionality:

* Add or update tests where appropriate
* Update documentation if the behavior is user-facing
* Follow the existing code style

### 4. Run checks

Rust:

```bash
cargo fmt --check
cargo clippy
cargo test
```

Frontend:

```bash
npm run lint
npm test
```

Run the relevant checks before opening a pull request.

### 5. Commit your changes

Use clear commit messages. Conventional Commits are preferred:

```text
feat: add message reactions
fix: handle invalid message ids
docs: update development setup
refactor: simplify message parsing
test: add websocket tests
chore: update dependencies
```

Keep commits focused when possible.

### 6. Push your branch

```bash
git push origin feature/add-example
```

### 7. Open a pull request

In the pull request description, briefly explain:

* What changed
* Why it was changed
* How it was tested
* Any related issues

For UI changes, include screenshots when they help explain the change.

## Development Setup

See [README.md](README.md) for the current development setup and project requirements.

## Code Style

### Rust

* Run `cargo fmt`
* Run `cargo clippy`
* Follow the existing project conventions
* Add documentation for public APIs where appropriate
* Prefer clear, idiomatic Rust over clever abstractions

### TypeScript / React

* Use TypeScript's strict mode
* Prefer functional components and hooks
* Follow the existing project conventions
* Run the project's lint and test commands before submitting a PR

### General

* Keep functions and modules focused
* Use descriptive names
* Avoid unnecessary abstractions
* Comment non-obvious code rather than restating what the code does

## Testing

Backend:

```bash
cd backend
cargo test
```

Frontend:

```bash
cd client
npm test
```

Run additional project-specific checks when needed.

## Documentation

User-facing changes should include the relevant documentation updates.

For larger architectural changes, update [ARCHITECTURE.md](ARCHITECTURE.md) when appropriate.

## Questions

For questions about development or contributing, use GitHub Discussions or open an issue.

Thanks for helping improve LieCord.

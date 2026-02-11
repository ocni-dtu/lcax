# Contributing to LCAx

Thank you for your interest in contributing to LCAx!
This document explains how to set up your environment and contribute changes to this repo.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Branch Naming Convention](#branch-naming-convention)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Pull Request Process](#pull-request-process)
- [Code Style and Linting](#code-style-and-linting)
- [Testing](#testing)

## Code of Conduct

Please be respectful and considerate of others when contributing to this project.

## Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/<your-username>/lcax.git
   cd lcax
   ```
3. Install dependencies:
   ```bash
   cargo install
   ```

## Branch Naming Convention

Use short, descriptive branch names grouped by type:

```
feature/<short-description>
fix/<short-description>
chore/<short-description>
docs/<short-description>
refactor/<short-description>
```

Examples:
- `feature/reusex-project-list`
- `fix/backend-auth-header`
- `chore/update-biome-config`

## Commit Message Guidelines

Write clear, concise commit messages that explain the changes you've made. Follow these guidelines:

1. Use the present tense ("Add feature" not "Added feature")
2. Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
3. Limit the first line to 72 characters or less
4. Reference issues and pull requests liberally after the first line
5. Consider starting the commit message with an applicable prefix:
   - `feat:` for new features
   - `fix:` for bug fixes
   - `docs:` for documentation changes
   - `style:` for formatting, etc.; no code change
   - `refactor:` for refactoring production code
   - `test:` for adding tests, refactoring test; no production code change
   - `chore:` for updating build tasks, package manager configs, etc.; no production code change

Example:
```
feat: add user authentication component

- Add login form
- Implement JWT token handling
- Add protected route component

Resolves #45
```

## Pull Request Process

1. Ensure your code follows the project's coding standards
2. Update documentation (`README.md` or relevant docs) if interfaces or workflows changed
3. Ensure the PR works for all affected apps/services in the monorepo
4. Include screenshots/GIFs if you've made UI changes (e.g., in `apps/reuse-x`)
5. Link the PR to any related issues (use `resolves #<issue-number>` where appropriate)
6. Request reviewers as defined in `CODEOWNERS` (if applicable)
7. Ensure CI checks pass (lint, type check, tests)
8. Address any requested changes from the code review
9. Once approved, a maintainer will merge the PR

## Code Style and Linting

This project uses Biome for linting/formatting and TypeScript for type checking.
Before submitting a PR, make sure your code passes all checks from the repo root:

```bash
cargo fmt --all
```

## Testing

Write tests for new features and ensure all tests pass before submitting a PR:

```bash
cargo test
```
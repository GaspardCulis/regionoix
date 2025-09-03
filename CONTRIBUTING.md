# Contributing to RegioNoix

## Getting started

### **Prerequisites**
- **Frontend**: Node.js (v18+), npm/yarn, Angular CLI
- **Backend**: Rust (latest stable), Cargo, Docker/Podman
- **Tools**: Git, GitHub account

### Frontend

#### Development server
To start a local development server, run:

```bash
ng serve
```

Once the server is running, open your browser and navigate to `http://localhost:4200/`. The application will automatically reload whenever you modify any of the source files.

#### Code scaffolding
Angular CLI includes powerful code scaffolding tools. To generate a new component, run:

```bash
ng generate component component-name
```

For a complete list of available schematics (such as `components`, `directives`, or `pipes`), run:

```bash
ng generate --help
```

#### Building
To build the project run:

```bash
ng build
```

This will compile your project and store the build artifacts in the `dist/` directory. By default, the production build optimizes your application for performance and speed.

### Backend

#### Running
To start the backend server, run:

```bash
cargo run
```

#### Code coverage
Requires [cargo-llvm-cov](https://crates.io/crates/cargo-llvm-cov).

```bash
cargo llvm-cov test
```


## Developement workflow

We try to apply the GitFlow workflow, specific features/issue fixes should be developped in their own branches. When the functionnality is developed, create a Pull Request and ask for reviewers.

## Conventions

All code and commit messages must be written in English.

### Commit convention

We use the [Conventional Commits v1.1.0](https://www.conventionalcommits.org/en/v1.0.0/) convention for commit messages.

### Branch naming

Branch names should be structured as follows:
```
<type>/short-scope-description
```
in Kebab Case. If the branch is dedicated to fixing a specific issue, `short-scope-description` can be replaced by the issue number.

### URL routes

All URL routes (pages and APIs) must be in Kebab Case (ex: `/api/customer-info`)

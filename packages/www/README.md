# react-ts-wasm-template

A simple web-app template using [Typescript](https://www.typescriptlang.org/), [React](https://reactjs.org/)
([Create React App](https://github.com/facebook/create-react-app)), [Redux](https://redux.js.org/), [WebAssembly (Rust)](https://rustwasm.github.io/) and Web Workers.

## Prerequisites

The Rust toolchain and `wasm-pack` is required to build the Rust/Webassembly code located in the rust-wasm directory. See the [`wasm-pack` documentation](https://rustwasm.github.io/wasm-pack/book/quickstart.html) for more information.

## Linting

Uses ESLint and prettier for linting and formatting.

## Usage

In the project directory, you can run:

### `npm run build-wasm`

Compiles the Rust code and builds WebAssembly modules for both the web-app (found in `rust-wasm/pkg`) and testin (found in `rust-wasm/pkg-node`). The latter is necessary because Jest runs in node.

### `npm run test-native`

Runs the native unit tests in the Rust code using `cargo test`.

### `npm install`

Installs the required packages from package.json.

### `npm start`

Runs the app in the development mode.<br />
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

The page will reload if you make edits.<br />
You will also see any lint errors in the console.

### `npm test`

Launches the test runner in the interactive watch mode.<br />
See the section about [running tests](https://facebook.github.io/create-react-app/docs/running-tests) for more information.

### `npm run build`

Builds the app for production to the `build` folder.<br />
It correctly bundles React in production mode and optimizes the build for the best performance.

The build is minified and the filenames include the hashes.<br />
Your app is ready to be deployed!

See the section about [deployment](https://facebook.github.io/create-react-app/docs/deployment) for more information.

### `npm run deploy`

Deploys the application to GitHub pages using gh-pages.

### `npm run eject`

**Note: this is a one-way operation. Once you `eject`, you can’t go back!**

If you aren’t satisfied with the build tool and configuration choices, you can `eject` at any time. This command will remove the single build dependency from your project.

Instead, it will copy all the configuration files and the transitive dependencies (webpack, Babel, ESLint, etc) right into your project so you have full control over them. All of the commands except `eject` will still work, but they will point to the copied scripts so you can tweak them. At this point you’re on your own.

You don’t have to ever use `eject`. The curated feature set is suitable for small and middle deployments, and you shouldn’t feel obligated to use this feature. However we understand that this tool wouldn’t be useful if you couldn’t customize it when you are ready for it.

## About

This project was bootstrapped with [Create React App](https://github.com/facebook/create-react-app), using the [Redux](https://redux.js.org/) and [Redux Toolkit](https://redux-toolkit.js.org/) template.

WebAssembly modules created with [Rust](https://www.rust-lang.org/) and [wasm-pack](https://rustwasm.github.io/wasm-pack/) using the [wasm-pack-template](https://github.com/rustwasm/wasm-pack).

Web Workers use [worker-loader](https://github.com/webpack-contrib/worker-loader) and [comlink](https://github.com/GoogleChromeLabs/comlink).

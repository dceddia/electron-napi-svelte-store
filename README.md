# Svelte Store Written in Rust

This repo shows a demo of an Electron app running a Svelte UI that reads
and writes a Svelte store, but with a twist: the store is implemented in
native Rust code 😎

Read the [blog post](https://daveceddia.com/svelte-store-in-rust/) to
learn how it came together.

## How to Run

Fair warning, it's not the most ergonomic project to develop with!

### Stuff You'll Need

- The [Rust toolchain](https://www.rust-lang.org/learn/get-started)
- Node

### Then,

- Clone this repo
- Run `npm install` in 3 separate places:
  - in the root folder, for Electron
  - in the `ui` folder, for Svelte
  - in the `bindings` folder, for the Rust module
- Run `npm start` from the root folder

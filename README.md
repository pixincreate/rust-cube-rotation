# Install trunk

[Trunk](https://trunkrs.dev/) is a WASM web application bundler for Rust. Trunk uses a simple, optional-config pattern for building & bundling WASM, JS snippets & other assets (images, css, scss) via a source HTML file

```bash
cargo install --locked trunk
```

## Check if trunk is installed

```bash
trunk --version
```

Other installation methods can be found [here](https://trunkrs.dev/#install)

## Add wasm target

```bash
rustup target add wasm32-unknown-unknown
```

## Run the trunk server

```bash
trunk serve
```

The page can be accessed at [localhost](http://localhost:8080/)

![demo gif](https://github.com/Narayanbhat166/cube_rotation/blob/main/cube_rotation.gif)

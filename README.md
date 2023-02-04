# Rust-Lang cube rotation

This is a learn-a-thon programme given by **Juspay**

## DISCLAIMER

THE CODE PROVIDED IS INTENDED TO BE USED AS A REFERENCE ONLY AND **SHOULD** NOT BE USED TO CHEAT IN THE CHALLENGE. CHEATING NOT ONLY GOES AGAINST THE SPIRIT OF THE CHALLENGE BUT ALSO TAKES AWAY FROM THE LEARNING EXPERIENCE. INSTEAD, I ENCOURAGE YOU TO USE YOUR OWN UNDERSTANDING AND CREATIVITY TO IMPLEMENT THE SOLUTION. REMEMBER, CODING IS A TOOL TO EXPRESS YOUR IDEAS, NOT A SUBSTITUTE FOR THINKING. **THE FEATURES ADDED ARE A RESULT OF MY OWN LOGIC AND UNDERSTANDING**, AND I BELIEVE THAT WITH A LITTLE EFFORT AND CREATIVITY, ANYONE CAN IMPLEMENT THEM. SO, USE THIS CODE TO **UNDERSTAND** THE CONCEPTS OR AS A **REFERENCE**, **NOT** TO CHEAT. I, [Pa1NarK](github.com/pixincreate) TAKES NO RESPONSIBILITY FOR THE MISERIES CAUSED BY THE USERS WHO USE **MY LOGIC** TO CHEAT IN THE LEARN-A-THON.

## Questions asked

**Q1.** Add two features (buttons) to the existing cube:

- reverses the rotation direction of that cube
- Increase or Decreases the velocity of rotation of that cube

**Q2.** Add / Remove the Cubes (button):

- Adds another cube
- Removes the cube

**Q3.** Add the code to make the cube Rotate in X, Y and Z direction as global buttons that rotates each of the cubes based on its respective velocity and direction

## Usage

1. Install trunk

> [Trunk](https://trunkrs.dev/) is a WASM web application bundler for Rust. Trunk uses a simple, optional-config pattern for building & bundling WASM, JS snippets & other assets (images, css, scss) via a source HTML file

```bash
cargo install --locked trunk
```

2. Check if trunk is installed

> If trunk is not installed, other installation methods can be found [here](https://trunkrs.dev/#install)

```bash
trunk --version
```

3. Add wasm target

```bash
rustup target add wasm32-unknown-unknown
```

4. Run the trunk server

```bash
trunk serve
```

____
The page can be accessed at `http://127.0.0.1:8080/`

## Demonstration

![demo gif](https://github.com/pixincreate/rust-cube-rotation/blob/main/cube_rotation.gif)

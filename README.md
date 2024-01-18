# Fifteen puzzle

Old Simple game where you have to order the numbers from 1 to 15 in a 4x4 grid.

![Screenshot](./images/game.png)

## About

This is a small pet project to learn [Slint](https://slint.dev) and [Rust](https://www.rust-lang.org/).

## Usage

I don't provide binaries (yet), so you have to build it yourself:

```bash
gh repo clone Light-Keeper/fifteen-puzzle
cd fifteen-puzzle
cargo build --release
```

Then you can run it:

```bash
./target/release/fifteen-puzzle
```

alternatively you can run the project directly with [taskfile](https://taskfile.dev) (if you have it installed):

```bash
task run
```

## Development

Look into [taskfile.yml](./taskfile.yml) for the available commands.

## Takeaways

1. [Slint](https://slint.dev) is a nice tradefoff between a full blown html/css-like frameworks and pure native solutions.
   UI is easy to update as features are added, but it does not provide a lot of customization options. Good for "any UI will do" kind of projects.

2. Resulting binary is quite small compared to Electron or Qt, but not as small as I'd imagine (10-20Mb).
3. Memory footprint of a runnig app is quite small (20-30Mb).

4. Rust's compiler is too restrictive. It's a good thing in the long run - the code is prepared for the worst case scenario. But it makes prototyping timeconsuming.

5. Cross-compilation is Sooooo slow! Build for Linux on Mac took 30 minutes on MacBook Pro 2019.

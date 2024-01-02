# Dungeon

### Installation + Setup

- Install Rust like described in the [Rust Documentation](https://doc.rust-lang.org/book/ch01-01-installation.html)
- The project uses [SDL2](https://github.com/libsdl-org/SDL/releases) and [SDL2_image](https://github.com/libsdl-org/SDL_image/releases) for rendering the game
  - If you get errors like `linking with 'link.exe' failed: exit code: 1120` or `fatal error LNK1181: cannot open input file 'SDL2.lib'` follow the steps described in this [GitHub Issue](https://github.com/PistonDevelopers/piston-examples/issues/391). You can download the packages from the sdl links above.

### Execution

- Navigate to the **app** folder and execute `cargo build`
- Execute `cargo run` to start the game

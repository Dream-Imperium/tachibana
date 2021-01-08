<h1 align="center">Tachibana</h1>

A work-in-progress 2D game framework purely written in the Rust programming language.

It adopts a model similar to osu!framework and Flutter, where widgets parent other widgets to build user interfaces. It also uses layout rules inspired from Godot.

A few noteworthy features:
- Lazy layout and sizing
- Game logic and rendering are done in separate threads (1000 updates/s, 120fps, hardcoded for now)
- No sweeping garbage collector

## Current state
For graphics, Tachibana utilises Skia on Vulkan. OpenGL/DirectX/Metal support not planned. Only `Immediate` present mode works well for now, `Mailbox`/`Fifo` performs poorly and a fix is not to come anytime soon.

Audio is not yet implemented and is not the main focus.

The repository currently contains both a binary and a library. The binary crate is the example code for now. To run it, execute `cargo run --release`.

## License
This project is unencumbered into the public domain. See the [LICENSE](LICENSE) file for more information.

## Contributing
This repository is actively reviewing and accepting pull requests and issues, especially those that improve performance, code quality and compile times.

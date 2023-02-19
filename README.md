# Tanglesync.com

This is the source for the website and demo on tanglesync.com

This code is messy but I'm making it available for those who want to learn from it.

## Structure

`index.ts` instantiates Tangle and sets up the host environment canvas function calls.

`rust_project` contains the code for the block-building game.

`rust_project/src/main.rs` contains the primary program logic.
    
`rust_project/src/mini_engine.rs` abstracts interacting with the host and running the game.
    

## Building

The build expects the [`tangle`](https://github.com/kettle11/tangle) and [`koi2`](https://github.com/kettle11/koi2) (koi, but named koi2 on your file system) repositories to be adjacent to the `tangle_website` repository.

You also need `npm` and Rust installed.  `devserver` is used as the test server which can be installed with `cargo install devserver`.

Once you have those things setup run the command `./run.sh` and visit `localhost:8080`.

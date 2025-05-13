## Developing

### With Nix
If you have nix and [nix-direnv](https://github.com/nix-community/nix-direnv) integrated with your shell simply type `direnv allow` and all shell commands will automatically run in an environment with the required build dependencies.

### Without Nix
Install the rust toolchain, just and cargo creates: clippy fmt nextest cargo-hack.
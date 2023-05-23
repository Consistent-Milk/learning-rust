Cargo Expand only works on Nightly builds (it offers more experimental features).
    Commands necessary to change to nightly build of rust (if not already installed).
    -> rustup install nightly
    -> rustup default nightly
    -> cargo install cargo-expand
    -> cargo expand --lib --tests (Expands macros used in all tests written in lib)
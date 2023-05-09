Running a specific program:

1. cd into any directory containing a Cargo.toml file
2. use terminal command: cargo run

The directory names start with an underscore due to rusts naming convention of projects/packages.

To enable rust-analyzer for a specific project in VSCode we have to modify settings.json inside .vscode folder and explicitly add the path to the projects' Cargo.toml file. (Maybe there is a way to automate this whenever cargo new is used to start a new project?)

Although using rustc for compiling and running programs consisting of only a single source code file may have been a better choice to save space, I think using cargo instead is necessary to understand the package/project management system of rust.

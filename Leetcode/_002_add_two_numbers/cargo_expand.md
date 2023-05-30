To expand the macro list!(), used in main.rs, the following commands were used:

1. cd ./src (otherwise the expanded source file is generated outside of src directory)

2. cargo expand --bin _002_add_two_numbers > expanded.rs

Otherwise the following single command can be used without changing directory:

-> cargo expand --bin _002_add_two_numbers > ./src/expanded.rs

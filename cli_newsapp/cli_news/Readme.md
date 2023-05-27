Based on: https://github.com/creativcoder/clinews

Changes made:
1. Changed a part of the source code so that Serde can deserialize null responses (check newsapi\src\lib.rs line: 44-59)
2. The crate crossterm seems to be provided as a module in termimad so used the modules and types provided by termimad crate instead.
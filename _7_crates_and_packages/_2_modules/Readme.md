Module structure used in this crate:
(The first underscore in function names were used to get rid of warning from rust-analyzer)
crate
└── front_of_house
├── hosting
│ ├── \_add_to_waitlist
│ └── \_seat_at_table
└── serving
├── \_take_order
├── \_serve_order
└── \_take_payment

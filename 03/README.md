# 03

This version adds structs for dates, events, and categories.
Dates use an enum type for the month.

The types have a derive attribute that automatically implements
the Debug trait, so that we can print them out with the 
`println!` macro using the `{:#?}` debug placeholder.
Later we will implement the `Display` trait for our types.

When you copy code from the book PDF, the indentation may (will?)
be messed up. If that should happen, re-format the source code
with `cargo fmt`.

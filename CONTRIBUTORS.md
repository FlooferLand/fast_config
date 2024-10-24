# Contributing

View a list of things that need to be done over at [TODO.md](./TODO.md)

Always write down any breaking changes inside [CONVERSION_TUTORIAL.md](./CONVERSION_TUTORIAL.md)

Use [test.sh](./scripts/test.sh) or [test.cmd](./scripts/test.cmd) to test the project

### Adding a new format

For the most part, you just need to modify [Cargo.toml](./Cargo.toml) and [format_dependant.rs](./src/format_dependant.rs) <br/>
There are other cases in the project _(such as checking if ANY of the features is enabled in `lib.rs`)_ where you will need to add your new format's feature.

I would recommend just doing a search for "TOML" and adding your new format wherever the others are referenced

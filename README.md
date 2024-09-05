# Remove expiration date policy in rooms

## What is this?

A command line tool to remove expiration date polices for rooms within a parent and their first level children.

This is based on [DRACOON Commander RS](https://github.com/unbekanntes-pferd/dccmd-rs) - an awesome project to use DRACOON via CLI and [dco3](https://github.com/unbekanntes-pferd/dco3) - a wrapper around the DRACOON API by [Octavio Simone](https://github.com/unbekanntes-pferd).

## How to use it?

Download the release or build from source.
Use the following command to remove expiration polices for rooms within a parent and their first level children.
The number is the parent room id of the rooms where the expiration date policy should be removed (set to 0).

```
remove-expiration run YOUR.DRACOON.COM/ 149
```

## Preconditions

- CLI user needs to be room admin to remove the expiration date

### Built with

This project makes use of several awesome crates and uses async Rust throughout the project.
Crates used:

- [reqwest](https://crates.io/crates/reqwest)
- [clap](https://crates.io/crates/clap)
- [console](https://crates.io/crates/console)
- [dialoguer](https://crates.io/crates/dialoguer)

Full dependency list: [Cargo.toml](Cargo.toml)

For all DRACOON operations `dco3` is used.

- [dco3](https://github.com/unbekanntes-pferd/dco3)

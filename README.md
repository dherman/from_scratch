# An experimental Node module implemented in Rust

"If you wish to make an apple pie from scratch, you must first invent the universe." - Carl Sagan

# Prereqs

I've only gotten so far, so all I can say is the environment I'm working on:

- stable Rust 1.2
- a 64-bit Mac
- Node 0.12.x

# Building

You can build the subcrates like so:

- `crates/nanners-sys`
```
% cargo build --release
```
- `crates/nanners`
```
% cargo build --release
```

The root crate is supposed to be built in two steps right now:
```
% cargo build --release
% npm install
```

# Testing

If everything builds OK, running
```
% node -e 'require("./")'
```
should print out `3.14` and exit successfully.

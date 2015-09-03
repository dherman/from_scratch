# An experimental Node module implemented in Rust

"If you wish to make an apple pie from scratch, you must first invent the universe." - Carl Sagan

# Prereqs

I've only gotten so far, so all I can say is the environment I'm working on:

- stable Rust 1.2
- a 64-bit Mac
- Node 0.12.x

# Building

For now, just use `make`. (I'm not convinced this is the best approach.)

# Testing

If everything builds OK, running
```
% node -e 'require("./")'
```
should print out `3.14` and exit successfully.

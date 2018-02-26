# russet

This document defines russet, but for versions <1.0.0 it's also a playground
for ideas.

This language is not meant to be a robust functional language, rather an
exercise in interpreter implementation. I have plans for implementing a
Rust-based Python interpreter with a friend, and I thought some practice would
be nice.

Russet will be dynamic, because dynamic is easy to implement. Although, having
optional static typing ala python or dart would be fun.

Russet will mostly be pure, save for I/O. This is for ease of implementation
mostly.

## Types

First off, primitives will be implemented.

The primitives are:
    - Integer (Signed word-sized integer)
    - Float (Double-precision float)
    - Boolean (true/false)
    - Tuple (Haskell-style tuple)
    - Linked lists.

Next on the list is Algebraic Data Types, like all functional languages. I'll
define these better once I get to it.

## Functions

Functions will be able to take in multiple positional arguments, and keyword
arguments might be fun.

Functions will not be able to modify their arguments in-place.

That's all I can think of at 2AM. More is coming sometime.

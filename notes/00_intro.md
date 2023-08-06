
## useful commands
```shell
# compile with rustc gen a binary file
rustc main.rs

# cargo commands
cargo new APP_NAME
cargo build
cargo run
cargo check
cargo build --release
cargo update
cargo doc --open

```

## Ownership
Ownership is a set of rules that govern how a Rust program manages memory
Rust uses an approach where the memory is managed through a system of ownership with a set of rules that the compiler checks.

### stack and heap
The stack stores values in the order it gets them and removes the values in the opposite order. This is referred to as last in, first out.
Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

The heap is less organized: when you put data on the heap, you request a certain amount of space. 
The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.

### Ownership rules
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

### Memory and Allocation

1. The memory must be requested from the memory allocator at runtime.
2. We need a way of returning this memory to the allocator when we’re done with our Type.

### Some types that implements Copy
1. All the integer types, such as `u32`.
2. The Boolean type, `bool`, with values `true` and `false`.
3. All the floating-point types, such as `f64`.
4. The character type, `char`.
5. Tuples, if they only contain types that also implement Copy. For example, (`i32`, `i32`) implements Copy, but (`i32`, `String`) does not.

## things to know about rust

* Statements are instructions that perform some action and do not return a value.
  Expressions evaluate to a resultant value. Let’s look at some examples.
* Calling the dbg! macro prints to the standard error console stream (stderr) 
* The &self is actually short for self: &Self
* the type Self is an alias for the type that the impl block is for
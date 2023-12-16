# Optimizing Rust

A lot of optimization techniques from other languages apply. In particular, C++ optimizations not only apply --- in most cases, LLVM is using the same optimizations. For example, "moving" out of a function triggers the same return-value optimizations as C++. Accessing the heap always requires a pointer-chase---with the same cache implications, so you want to avoid pointers to pointers.

The golden rule also applies: profile before you micro-optimize. Compilers are amazingly smart.

A few of the Best Practices also help: [Favor Iterators](./Iterators.md), [Minimize Cloning](./Clone.md), [Don't Emulate OOP](./OOPs.md), [Don't Reference Count Everything](./Rc.md). Iterators in particular tend to optimize better than equivalent loops. And its very easy to try to "beat" the borrow checker by cloning/reference counting---but doing so comes at a cost. Deciding where to pay the cost is up to you.


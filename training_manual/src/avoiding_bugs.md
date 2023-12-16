# Avoiding Bugs

Much of this was covered in the Best Practices section, so we won't belabor it. In particular:

* [Let the Type System Help You](./TypeSystem.md)
* [Don't Forget Floating Point Issues](./Floats.md)

In general, Rust can help you stay productive if you embrace its rules:

* Minimize `unsafe`, and wrap it in safe interfaces. Document the need for the lack of safety.
* Do run in `debug` mode periodically, to catch overflows and out-of-bounds accesses.
* Embrace well-contained code with readable functions.
* Embrace the `Result` type, and check your preconditions! If this is too slow in production, wrap your checks in conditional compilation and make sure that you test them.
* Unit test everything that makes sense to unit test.
* Don't opt out of safety unless *you really need to*.

These are similar to C++ guidelines, with which you should be familiar.
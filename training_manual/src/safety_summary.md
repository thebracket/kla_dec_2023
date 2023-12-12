# Safety Wrap-Up

So we've taken a deep-dive into Rust's safety promises. Rust offers a significantly safer experience than other systems languages, while remaining deterministic and without a garbage collector:

* Type conversion is explicit, and offers protections against inadvertent data loss when you cast an `i32` into an `i16`.
* Overflow behavior is explicit, and trapped at runtime in `Debug` mode.
* Checked arithmetic lets you easily act on common overflow, division by zero and similar issues rather than crashing.
* Out-of-bounds access is checked, preventing common bugs and security vulnerabilities.
* No null pointers, and therefore no null pointer references.
* Move, reference and copy are explicit. You can't accidentally pass-by-value.
* Immutable by default makes it harder to accidentally apply side-effects in your code.
* RAII makes it difficult to accidentally leak memory or other resources.
* The borrow checker prevents aliasing and potentially confusing results.
* The borrow checker prevents subtle bugs like iterator invalidation, albeit occasionally at the cost of making an operation difficult.
* Lifetime protection prevents dangling pointers, use-after-free and use-after-move.
* Explicit `Send` protection ensures that when you access data between threads, the data won't be inadvertently copied, is safe to send between thread contexts.
* Explicit `Sync` protection makes data-races practically impossible.
* You *can* opt-out of the extra safety checks with the `unsafe` tag---and should be very careful doing so.

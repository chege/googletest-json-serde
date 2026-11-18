# Follow-ups

## Reduce recursion in array matcher macros

Very long lists of complex expressions and deeply nested arrays can still
exceed the calling crate's default macro recursion limit. Inner array literals
are now parsed in batches, including in `j::each!` and `j::pat!`.

The current workaround is to increase the limit in the calling crate, such as
with `#![recursion_limit = "1024"]`.

Investigate reducing expansion depth further without changing array literal
semantics, Rust array expression support, matcher composition, or duplicate
handling. Add consumer-side compilation tests for long mixed argument lists
and for the documented workaround.

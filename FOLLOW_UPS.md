# Follow-ups

## Reduce recursion in array matcher macros

Very long lists of complex expressions and deeply nested arrays or object
patterns can still exceed the calling crate's default macro recursion limit.
Inner array literals, array argument lists, and simple object fields are parsed
in batches.

The current workaround is to increase the limit in the calling crate, such as
with `#![recursion_limit = "1024"]`.

Investigate reducing expansion depth further without changing array literal
semantics, Rust array expression support, matcher composition, or duplicate
handling. Add a consumer-side compilation test for the documented workaround.

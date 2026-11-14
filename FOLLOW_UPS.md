# Follow-ups

## Reduce recursion in array matcher macros

Long inner array literals and long lists of complex expressions can exceed the
calling crate's default macro recursion limit. For example, an inner array of
128 elements can fail while expanding `__json_array_value!`. This also affects
array literals in `j::each!` and `j::pat!`.

The current workaround is to increase the limit in the calling crate, such as
with `#![recursion_limit = "1024"]`.

Investigate reducing expansion depth without changing array literal semantics,
Rust array expression support, matcher composition, or duplicate handling. Add
consumer-side compilation tests for long inner literals and mixed argument
lists at the default limit, as well as a test of the documented workaround.

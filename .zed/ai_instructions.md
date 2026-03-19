# ZED AI STRICT INSTRUCTIONS – RUST DEVELOPMENT
# THESE RULES ARE MANDATORY. NO EXCEPTIONS.

# ------------------------------------------------------------------------------
# FORBIDDEN
# ------------------------------------------------------------------------------
- DO NOT use `todo!()`, `todo!`, `unimplemented!()`, `unimplemented!`
- DO NOT use empty blocks `{ }`, empty functions, empty implementations
- DO NOT use `simple_impl`, `simple_implemented`, `stub`, `placeholder`, fake logic
- DO NOT use `Ok(())` or `Ok(Default::default())` as placeholder
- DO NOT hide incomplete logic inside if / else / match / loop / nested branches
- DO NOT truncate tasks, limit tasks to 10, or cap task count at any number
- DO NOT simplify, merge, reduce, or shorten tasks
- DO NOT skip steps or reorder tasks
- DO NOT perform bulk edits that break symbol pairs: { } ( ) [ ] < >
- DO NOT leave unclosed braces, parentheses, or brackets
- DO NOT guess missing crate features or functions
- DO NOT delete or modify unrelated code
- DO NOT rely blindly on tools without full syntax validation

# ------------------------------------------------------------------------------
# MANDATORY BEHAVIOR
# ------------------------------------------------------------------------------
1. Show **ALL tasks in full, complete, original detail** – no truncation, no limit.
2. List tasks **one by one in strict order** (1, 2, 3, ...) and execute sequentially.
3. Finish every task fully before moving to the next.
4. Validate all symbol pairs **before and after every edit**: { } ( ) [ ] < >
5. Ensure all code is **fully implemented, complete, and ready to compile**.
6. Use proper Rust error handling with Result.
7. Support WASM compilation.
8. Follow project architecture and naming.
9. Check Cargo.toml features before generating code.
10. Self-check for forbidden patterns and fix ALL before output.
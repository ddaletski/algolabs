# Leetcode

My leetcode solutions in Rust


## Usage

Every directory under `problems/` is a cargo project which contains some basic test cases.

The whole project is a cargo workspace,
so all solutions can be tested using `cargo test` from repository root


## Common

The workspace contains a `common` crate which is used in tests
and also contains leetcode-compatible definitions of common data structures,
like `TreeNode` and `ListNode`

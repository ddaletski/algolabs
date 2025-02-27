# Leetcode

My leetcode solutions in Rust

## Usage

Every directory under `problems/` is a cargo project which contains some basic test cases.

The whole project is a cargo workspace,
so all solutions can be tested using `cargo test` from repository root

## Adding a problem

Use [this script](./add_problem.py) to initialize new problem quickly]. Example usage:

```shell
python add-problem.py leetcode "https://leetcode.com/problems/length-of-longest-fibonacci-subsequence"
```

## Common

The workspace contains a `common` crate which is used in tests
and also contains leetcode-compatible definitions of common data structures,
like `TreeNode` and `ListNode`

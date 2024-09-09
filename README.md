```bash
cargo +nightly contract build
```

```bash
cargo test
```

```bash
cargo contract instantiate \
    --constructor new \
    --suri //Alice \
    --args '[]' '[]' \
    --skip-confirm --execute
```

```bash
cargo contract call \
    --contract 5Gcs6Gd73e5UkwMiuJGCghjJvdYdshPMsusuKVW1gNZ4H3yb \
    --message create_book \
    --suri //Alice \
    --args '"Book2"' '"Categor21"' '"Autho21"' '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY' \
    --skip-confirm --execute
```

```bash
cargo contract call \
    --contract 5Gcs6Gd73e5UkwMiuJGCghjJvdYdshPMsusuKVW1gNZ4H3yb \
    --message get_books_by_owner_id \
    --suri //Alice \
    --args '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY' \
    --skip-dry-run
```

```bash
cargo contract call \
    --contract 5CN1ruHQ6fTfFGLUgL4Q4PxbAG66xVRrVZCnFUvuYGJM8NZm \
    --message update_book \
    --suri //Alice \
    --args '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY' 0 'Some("NewName")' 'Some("NewCategory")' 'Some("NewAuthor")' \
    --skip-confirm --execute
```
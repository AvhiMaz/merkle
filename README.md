# merkle

<img width="1532" height="1091" alt="image" src="https://github.com/user-attachments/assets/85bb0365-7152-429b-a44a-14c538f5bd18" />

an on-chain incremental merkle tree built on solana, implemented in two frameworks for comparison.

append-only tree of depth 20 (up to 1,048,576 leaves). only stores `log2(n)` hashes on-chain -the frontier of filled subtrees -so insert and verify are both `O(depth)`.


## implementations

| | quasar | anchor |
|---|---|---|
| framework | [quasar](https://github.com/blueshift-gg/quasar) | [anchor](https://github.com/solana-foundation/anchor) |
| tests | quasar-svm | mollusk-svm |

## compute units

| instruction | quasar | anchor |
|---|---|---|
| initialize | 1,801 | 8,294 |
| insert | 3,676 | 9,405 |
| verify | 3,218 | 5,426 |

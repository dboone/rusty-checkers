# rusty-checkers

Checkers game implemented in Rust.

## Implementation

* The checkers `board` is simply a collection of tiles. A `tile` can be either `EmptyTile` or `OccupiedTile`.
* Checkers `pieces` will eventually occupy tiles. A `piece` can be either `ManPiece` or `KingPiece`.

## Long term goals

* OpenGL based visualization using `glutin`
* Refine abstractions to support chess

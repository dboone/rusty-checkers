# rusty-checkers

<a href="https://travis-ci.org/dboone/rusty-checkers"><img src="https://travis-ci.org/dboone/rusty-checkers.svg"/></a>

Checkers game implemented in Rust.

## Implementation
### Modules
* `board` - contains the `Board` struct
* `ai` - contains logic for generating valid moves
* `tile` - contains the `Tile` trait, `EmptyTile` and `OccupiedTile` structs
* `piece` - contains the `Piece` trait, `ManPiece` and `KingPiece` structs
* `player` - contains the `Player` struct
* `input` - converts a `String` to a sequence of move positions

### Details
* The checkers `board` is simply a collection of tiles. A `tile` can be either `EmptyTile` or `OccupiedTile`.
* Checkers `pieces` occupy tiles. A `piece` can be either `ManPiece` or `KingPiece`.
* Moves are either `SimpleMoves` or `JumpMoves`. If a user has a `JumpMove` available, they are required to take it.

## Long term goals
* OpenGL based visualization using `glutin`
* Refine abstractions to support chess

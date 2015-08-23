# rusty-checkers

<a href="https://travis-ci.org/dboone/rusty-checkers"><img src="https://travis-ci.org/dboone/rusty-checkers.svg"/></a>

Checkers game implemented in Rust.

<img align="right" src="http://imgur.com/zleHaok.gif" alt="checkers in 20 moves"/>

### Game Play
The game play consists of each player entering moves until an end game state is reached. The game can be quit at any time by entering `q` or `Q` instead of a move.

### Board
The board is a regulation 8 by 8 checkers boad. The tiles are indexed using File and Rank. The board is labeled with File `A` through `H` and Rank `1` through `8`, with board position `A1` in the lower lefthand corner of the board. Board positions must consist of File *then* Rank.

```
a3     // valid
h6     // valid

3a     // invalid, file must be first
6h     // invalid, file must be first
```

### Moves
Moves consist of at least two board positions. The first board position specifies the piece to be moved and the second board position specifies the destination of the piece. Additional destinations can be specified to make multiple jumps (double, triple, etc.).

```
a3 b4        // valid, move a3 to b4 (simple move)
a5 c3 e1     // valid, move a5 to c3, e1 (double jump)
e7 c5 a3 c1  // valid, move e7 to c5, a3, c1 (triple jump)

a3           // invalid, must specify destination
a3b4         // invalid, missing space
```

### Error Messages
**Illegal move**: the specified move is illegal. For example, it is illegal to:
* move a piece that is not yours/doesn't exist
* move/jump to a tile that is not on the board
* move/jump to a tile that is occupied
* jump your own tile
* move men backwards

**Must take jump**: at least one jump is available to the current player. Players are required to take jumps they are presented with. If a multi-jump is available, players are only required to make the first part of the jump. The remainder of the jump sequence is left to the player's discretion.

**You must specify at least two board positions**: each move must consist of at least two board positions. See the **Moves** section above.
```
> a3
*** You must specify at least two board positions
> a3 b4
[OK]
```

**Board position must specify file/rank**: each board position must contain a file then rank.
```
> a
*** Board position 'a' must specify rank
> 3
*** Board position '3' must specify file
> a3 b4
[OK]
```

**Rank cannot be zero**: the rank must be at least 1.
```
> b0
*** Rank cannot be zero: 'b0'
> b1
[OK]
```

**Board position contains invalid character**: only alphanumeric characters are valid. Special characters and punctuation are not allowed. Additionally alpabetic characters are not allowed after numeric characters because File must be specified before Rank. The invalid character will be presented to the player:
```
> a$ b4
*** Board position 'a$' contains invalid character '$'
> 3a 4b
*** Board position '3a' contains invalid character 'a'
*** Board position '4b' contains invalid character 'b'
> a3 b4
[OK]
```

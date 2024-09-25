# **Example**
```rust
use caspervk_chess::{board_pos_to_index, BoardState, Game};
let mut game = Game::new();
let possible_movements = game.get_position_possible_movements(board_pos_to_index("a2".to_string()));
let board_state = game.do_move(board_pos_to_index(a2), board_pos_to_index("a4".to_string()));
```
# **Structs**

# **Piece**
## **Possible values**
    King
    Queen
    Rook
    Bishop
    Knight
    Pawn
    None
# **BoardState**
## **Possible values**
    Default
    Checked(Side)
    CheckMated(Side)
    WhiteLoseByTime
    BlackLoseByTime
    DrawBy50Rule
    DrawByStaleMate
    WhitePromotion
    BlackPromotion
# **Side**
## **Possible values**
    White
    Black
    None
# **Variables**
All of the following variables can be accessed from the Game Object.
### **board_pieces: [Piece; 64]**
Contains all of the current pieces on the board. Index 0 contains the board position of A1, Index 1 A2, Index 8 A2, and so on...
### **board_pieces_sides: [Side; 64]**
Contains the sides of all the current pieces on the board. Index positions are same as listed above.
### **curr_turn: Side**
### **last_move_origin: i8** 
Array index of where the last move was made from
### **last_move_target: i8**
Array index of where the last move was made to

# **Game's methods**
# **get_position_possible_movements**
## **Parameters**
### ```position: i8```
array index of the piece
## **Return value**
###  ```Vec<i8>```
# **do_move**
## **Parameters**
### ```origin: i8```
### ```target: i8```
## **Return value**
###  ```BoardState```
Returns the current state of the board that was updated after the move was made.
# **choose_promotion_piece**
Chooses the piece that the pawn being promoted will become. It is important that this method is called after a **do_move** returns either **WhitePromotion** or **BlackPromotion**.
## **Parameters**
### ```piece: Piece```
### **Return value**
### ```BoardState```
Returns the state of the board after the promotion was made.
# **request_draw**
Requests draw based on threefold repetition.
## **Return value**
### ```bool```
If request was rightful
# **Utility functions**
# **board_pos_to_index** 
Converts a board position to an array index. For example "a1" converts to 0.
## **Parameters**
### ```board_pos: String```
## **Return value**
### ```i8```

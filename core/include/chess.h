#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
  Black,
  White,
} Color;

typedef enum {
  Pawn,
  Bishop,
  Knight,
  Rook,
  King,
  Queen,
} PieceType;

typedef struct Move Move;

typedef struct Chessboard Chessboard;

typedef struct {
  intptr_t _0;
  intptr_t _1;
} Position;

typedef struct {
  PieceType ty;
  Color color;
} Piece;

typedef struct {
  const Move *ptr;
  size_t len;
} PossibleMoves;

void board_destroy_piece_at(Chessboard *board, Position pos);

const Piece *board_get_piece_at(const Chessboard *board, Position pos);

PossibleMoves board_get_possible_moves(const Chessboard *board, Position pos);

Chessboard board_new(void);

void board_set_piece_at(Chessboard *board, Position pos, Piece piece);

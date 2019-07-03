#ifndef LIXCHESS_CLIENT_RENDER_H
#define LIXCHESS_CLIENT_RENDER_H

#include <main.h>

#define ERR_RENDER_GLAD_INIT 1

void render_init(State *state, int *error);

void render_chessboard(State *state, int *error);

void render_close();

#endif //LIXCHESS_CLIENT_RENDER_H

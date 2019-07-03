#ifndef LIXCHESS_CLIENT_RENDER_H
#define LIXCHESS_CLIENT_RENDER_H

#include <main.h>

typedef struct RenderState RenderState;

#define ERR_RENDER_GLAD_INIT 1

RenderState *render_init(int *error);

void render_chessboard(State *state, RenderState *rstate, int *error);

#endif //LIXCHESS_CLIENT_RENDER_H

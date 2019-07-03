#ifndef LIXCHESS_CLIENT_MAIN_H
#define LIXCHESS_CLIENT_MAIN_H
#include <render.h>

typedef struct State State;

struct State {
    RenderState *rstate;
    int running;
};


#endif //LIXCHESS_CLIENT_MAIN_H

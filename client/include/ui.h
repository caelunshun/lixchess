#ifndef LIXCHESS_CLIENT_UI_H
#define LIXCHESS_CLIENT_UI_H

#include "main.h"

#define ERR_UI_SHADER_FAILURE 1

typedef struct UiState UiState;

/* Must be called after render_init so that OpenGL
 * is initialized
 * */
UiState *ui_init(int *err);

void render_ui(State *state, UiState *ustate, int *err);

typedef enum UiStage {
    LOGIN_PAGE,
    SERVER_LIST,
    GAME_LIST,
    GAME,
} UiStage;

#endif //LIXCHESS_CLIENT_UI_H

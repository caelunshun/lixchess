#include <main.h>
#include <stdio.h>
#include <render.h>
#include <ui.h>

int main_loop(State *state, RenderState *rstate);

void handle_render_err(int errno) {
    switch (errno) {
        case ERR_RENDER_GLAD_INIT:
            printf("Failed to load GLAD");
            break;
        default:
            printf("Unknown error occurred while initialising OpenGL: %d", errno);
            break;
    }
}

int main() {
    printf("Initializing. Please wait...\n");

    int rerr = 0;
    RenderState *rstate= render_init(&rerr);

    if (rerr != 0) {
        handle_render_err(rerr);
        return 1;
    }

    State state;
    state.running = true;
    state.in_game = false;

    return main_loop(&state, &rstate);
}

int main_loop(State *state, RenderState *rstate) {
    while (state->running) {
        if (state->in_game) {
            int err = 0;
            render_chessboard(state, rstate, &err);

            if (err != 0) {
                handle_render_err(err);
                return 1;
            }
        }
    }

    printf("Shutting down. Goodbye!\n");
    return 0;
}
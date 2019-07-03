#include <main.h>
#include <stdio.h>
#include <render.h>

void main_loop(State *state);

void handle_render_err(int errno) {
    switch (errno) {
        case RENDER_GLAD_INIT_FAILED:
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
    state.rstate = rstate;
    state.running = 1;

    main_loop(&state);
}

void main_loop(State *state) {
    while (state->running) {
        r
    }
}


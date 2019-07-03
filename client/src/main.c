#include <main.h>
#include <stdio.h>
#include <render.h>
#include <ui.h>

int main_loop(State *state, UiState *ustate);

void handle_render_err(int errno) {
    switch (errno) {
        case ERR_RENDER_GLAD_INIT:
            printf("Failed to load GLAD");
            break;
        default:
            printf("Unknown error occurred while initialising OpenGL: %d\n", errno);
            break;
    }
}

void handle_ui_err(int errno) {
    switch (errno) {
        default:
            printf("Unknown error occurred with UI rendering\n");
            break;
    }
}

int main() {
    printf("Initializing. Please wait...\n");

    State state;
    state.running = true;
    state.in_game = true; // TODO set to false

    int err = 0;
    render_init(&state, &err);

    if (err != 0) {
        handle_render_err(err);
        return 1;
    }

    UiState *ustate = ui_init(&err);

    if (err != 0) {
        handle_ui_err(err);
        return 1;
    }

    glClearColor(0.1, 0.1, 0.15, 1.0);

    int res = main_loop(&state, ustate);
    render_close();

    return res;
}

int main_loop(State *state, UiState *ustate) {
    printf("Running main loop.\n");
    while (state->running) {
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        if (state->in_game) {
            int err = 0;
            render_chessboard(state, &err);

            if (err != 0) {
                handle_render_err(err);
                return 1;
            }
        }
        int err = 0;
        render_ui(state, ustate, &err);

        if (err != 0) {
            printf("Error rendering UI");
            return 1;
        }

        glfwSwapBuffers(state->window);
        glfwPollEvents();

        if (glfwWindowShouldClose(state->window)) {
            state->running = false;
        }
    }

    printf("Shutting down. Goodbye.\n");
    return 0;
}
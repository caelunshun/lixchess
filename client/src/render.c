#include <render.h>
#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <stdlib.h>
#include <main.h>

#define WINDOW_WIDTH 1920/2
#define WINDOW_HEIGHT 1080/2
#define WINDOW_TITLE "LixChess"

struct RenderState {
    GLFWwindow *window;
    int should_close;
};

void framebuffer_size_cb(GLFWwindow *window, int width, int height) {
    glViewport(0, 0, width, height);
}

RenderState *render_init(int *error) {
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    RenderState *state = malloc(sizeof(state));

    state->window = glfwCreateWindow(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, NULL, NULL);
    state->should_close = 0;

    glfwMakeContextCurrent(state->window);

    if (!gladLoadGLLoader((GLADloadproc) glfwGetProcAddress)) {
        *error = ERR_RENDER_GLAD_INIT;
        return NULL;
    }

    glViewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

    glfwSetFramebufferSizeCallback(state->window, framebuffer_size_cb);

    return state;
}

void render_chessboard(State *state, RenderState *rstate, int *error) {
    if (glfwWindowShouldClose(rstate->window)) {
        rstate->should_close = 1;
        return;
    }

    glfwSwapBuffers(rstate->window);
    glfwPollEvents();
}
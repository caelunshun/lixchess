#include <render.h>
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

void render_init(State *state, int *error) {
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    state->window = glfwCreateWindow(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, NULL, NULL);

    glfwMakeContextCurrent(state->window);

    if (!gladLoadGLLoader((GLADloadproc) glfwGetProcAddress)) {
        *error = ERR_RENDER_GLAD_INIT;
        return;
    }

    glViewport(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);

    glfwSetFramebufferSizeCallback(state->window, framebuffer_size_cb);
}

void render_chessboard(State *state, int *error) {

}

void render_close() {
    glfwTerminate();
}
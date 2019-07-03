#ifndef LIXCHESS_CLIENT_MAIN_H
#define LIXCHESS_CLIENT_MAIN_H

#include <stdbool.h>
#include <GLFW/glfw3.h>
#include <glad/glad.h>

typedef struct State State;

struct State {
    bool running;
    bool in_game;

    GLFWwindow *window;
};


#endif //LIXCHESS_CLIENT_MAIN_H

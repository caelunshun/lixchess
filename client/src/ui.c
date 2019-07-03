#include <ui.h>
#include <main.h>
#include <glad/glad.h>

#define NK_IMPLEMENTATION
#define NK_INCLUDE_FIXED_TYPES
#define NK_INCLUDE_STANDARD_IO
#define NK_INCLUDE_DEFAULT_ALLOCATOR
#define NK_INCLUDE_VERTEX_BUFFER_OUTPUT
#define NK_INCLUDE_FONT_BAKING

#include <nuklear.h>
#include <util.h>
#include <shader.h>

struct UiState {
    struct nk_context ctx;
    struct nk_buffer cmd_buf;

    Shader shader;
    UiStage stage;
};

UiState *ui_init(int *err) {
    UiState *result = malloc(sizeof(UiState));

    Shader shader;
    int serr = 0;
    shader_create("assets/shaders/ui.vert.glsl", "assets/shaders/ui.frag.glsl", &shader, &serr);
    result->shader = shader;

    if (serr != 0) {
        *err = ERR_UI_SHADER_FAILURE;
        return result;
    }

}

#include <ui.h>
#include <main.h>

#define NK_INCLUDE_FIXED_TYPES
#define NK_INCLUDE_STANDARD_IO
#define NK_INCLUDE_DEFAULT_ALLOCATOR
#define NK_INCLUDE_VERTEX_BUFFER_OUTPUT
#define NK_INCLUDE_FONT_BAKING
#define NK_IMPLEMENTATION
#include <nuklear.h>
#include <util.h>
#include <shader.h>

typedef struct {
    vec2 position;
    vec2 uv;
    vec4 color;
} Vertex;

struct UiState {
    struct nk_context ctx;
    struct nk_buffer cmd_buf;

    Shader shader;
    UiStage stage;

    GLuint vbo;
    GLuint ebo;
    GLuint vao;

    GLuint uniform_mat;
    GLuint uniform_texture;
    GLuint attr_position;
    GLuint attr_uv;
    GLuint attr_color;
};

void ui_init_buffers(UiState *ustate, int *err);

UiState *ui_init(int *err) {
    UiState *state = malloc(sizeof(UiState));

    Shader shader;
    int serr = 0;
    shader_create("assets/shaders/ui.vert.glsl", "assets/shaders/ui.frag.glsl", &shader, &serr);
    state->shader = shader;

    if (serr != 0) {
        *err = ERR_UI_SHADER_FAILURE;
        return state;
    }

    state->stage = LOGIN_PAGE;

    nk_init_default(&state->ctx, 0);

    state->uniform_mat = glGetUniformLocation(shader.program_id, "matrix");
    state->uniform_texture = glGetUniformLocation(shader.program_id, "tex");

    state->attr_position = glGetAttribLocation(shader.program_id, "pos");
    state->attr_uv = glGetAttribLocation(shader.program_id, "uv_coord");
    state->attr_color = glGetAttribLocation(shader.program_id, "color");

    ui_init_buffers(state, err);

    if (*err != 0) {
        return state; // Error - end here
    }

    return state;
}

void ui_init_buffers(UiState *state, int *err) {
    GLsizei size = sizeof(Vertex);
    size_t pos_offset = offsetof(Vertex, position);
    size_t uv_offset = offsetof(Vertex, uv);
    size_t color_offset = offsetof(Vertex, color);

    glGenBuffers(1, &state->vbo);
    glGenBuffers(1, &state->ebo);
    glGenVertexArrays(1, &state->vao);

    glBindVertexArray(state->vao);
    glBindBuffer(GL_ARRAY_BUFFER, state->vbo);
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, state->ebo);

    glEnableVertexAttribArray(state->attr_position);
    glEnableVertexAttribArray(state->attr_uv);
    glEnableVertexAttribArray(state->attr_color);

    glVertexAttribPointer(state->attr_position, 2, GL_FLOAT, GL_FALSE, size, (void*) pos_offset);
    glVertexAttribPointer(state->attr_uv, 2, GL_FLOAT, GL_FALSE, size, (void*) uv_offset);
    glVertexAttribPointer(state->attr_color, 2, GL_FLOAT, GL_FALSE, size, (void*) color_offset);
}

void render_ui(State *state, UiState *ustate, int *err) {

}
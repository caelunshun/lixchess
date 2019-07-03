#include <ui.h>
#include <glad/glad.h>

#define NK_IMPLEMENTATION
#define NK_INCLUDE_FIXED_TYPES
#define NK_INCLUDE_STANDARD_IO
#define NK_INCLUDE_DEFAULT_ALLOCATOR
#define NK_INCLUDE_VERTEX_BUFFER_OUTPUT
#define NK_INCLUDE_FONT_BAKING

#include <nuklear.h>
#include <util.h>

struct UiState {
    struct nk_context ctx;
    UiStage stage;
};

UiState *ui_init(int *err) {
    
}

#ifndef LIXCHESS_CLIENT_SHADER_H
#define LIXCHESS_CLIENT_SHADER_H
#include <glad/glad.h>
#include <cglm/vec3.h>

#define ERR_SHADER_NOT_FOUND 1
#define ERR_SHADER_COMPILE_ERROR 2
#define ERR_SHADER_LINK_ERROR 3

typedef struct {
    GLuint program_id;
} Shader;

void shader_create(const char *vert, const char *frag, Shader *shader, int *err);

#endif //LIXCHESS_CLIENT_SHADER_H

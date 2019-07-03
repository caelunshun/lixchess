#include <shader.h>
#include <util.h>

void shader_create(const char *vert, const char *frag, Shader *shader, int *err) {
    int ferr = 0;
    FileBuf vertf = read_file(vert, &ferr);
    if (ferr != 0) {
        *err = ERR_SHADER_NOT_FOUND;
        return;
    }

    FileBuf fragf = read_file(frag, &ferr);
    if (ferr != 0) {
        *err = ERR_SHADER_NOT_FOUND;
        return;
    }

    GLuint vertid, fragid;

    vertid = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertid, 1, vertf.buf, NULL);
    glCompileShader(vertid);

    int success;
    glGetShaderiv(vertid, GL_COMPILE_STATUS, &success);
    if (!success) {
        *err = ERR_SHADER_COMPILE_ERROR;
        return;
    }

    fragid = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragid, 1, fragf.buf, NULL);
    glCompileShader(fragid);

    glGetShaderiv(fragid, GL_COMPILE_STATUS, &success);
    if (!success) {
        *err = ERR_SHADER_COMPILE_ERROR;
        return;
    }

    GLuint program_id = glCreateProgram();
    glAttachShader(program_id, vertid);
    glAttachShader(program_id, fragid);
    glLinkProgram(program_id);

    glGetProgramiv(program_id, GL_LINK_STATUS, &success);
    if (!success) {
        *err = ERR_SHADER_LINK_ERROR;
        return;
    }

    glDeleteShader(vertid);
    glDeleteShader(fragid);

    shader->program_id = program_id;
}
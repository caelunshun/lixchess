#ifndef LIXCHESS_CLIENT_UTIL_H
#define LIXCHESS_CLIENT_UTIL_H

#include <stddef.h>

#define ERR_FILE_NOT_EXISTS 1
#define ERR_IO 2

typedef struct {
    size_t len;
    char *buf;
} FileBuf;

FileBuf read_file(char *path, size_t path_len, int *err);

#endif //LIXCHESS_CLIENT_UTIL_H

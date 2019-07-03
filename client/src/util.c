#include <util.h>

#include <stdlib.h>
#include <stdio.h>

FileBuf read_file(char *path, size_t path_len, int *err) {
    char *buf = NULL;
    FileBuf res;

    FILE *f = fopen(path, "r");
    if (f == NULL) {
        *err = ERR_FILE_NOT_EXISTS;
        return res;
    }

    long size = ftell(f);
    if (size == -1) {
        *err = ERR_IO;
        return res;
    }

    buf = malloc(sizeof(char) * (size + 1));

    if (fseek(f, 0L, SEEK_SET) != 0) {
        *err = ERR_IO;
        return res;
    }

    size_t len = fread(buf, sizeof(char), size, f);
    if (ferror(f) != 0) {
        *err = ERR_IO;
        return res;
    }

    buf[len + 1] = '\0';

    fclose(f);

    res.buf = buf;
    res.len = len;
    return res;
}
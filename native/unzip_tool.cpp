#include <cstring>
#include <sys/stat.h>
#include <sys/types.h>

#ifdef _WIN32
#include "dirent/include/dirent.h"
#include <direct.h>
#else
#include <dirent.h>
#endif

#include "unzip_tool.h"

#ifdef _WIN32
#include "zlib/contrib/minizip/zip.h"
#include "zlib/contrib/minizip/unzip.h"
#else
#include "minizip/zip.h"
#include "minizip/unzip.h"
#endif

enum UnzipResult {
  UNZIP_OK,
  OPEN_ZIP_FILE_FAILED,
  GET_ZIP_INFO_FAILED,
  GET_FILE_INFO_FAILED,
  GO_TO_NEXT_FILE_FAILED,
  CREATE_DIRECTORY_FAILED,
  PATH_TOO_LONG,
  OPEN_ZIP_ENTRY_FAILED,
  READ_ZIP_ENTRY_FAILED,
  CLOSE_ZIP_ENTRY_FAILED,
  CLOSE_ZIP_FILE_FAILED,
  OTHER_ERROR,
};

const int MAX_FILENAME_LEN = 8192;
const int BUFFER_SIZE = 8192;

int mkdirs(const char *dir);

int dump_entry(unzFile file, const char *filename, const char *target_path) {
  int return_result = 0;
  int err = unzOpenCurrentFile(file);
  if (err != UNZ_OK) {
    printf("error: open zip entry %s failed\n", filename);
    return OPEN_ZIP_ENTRY_FAILED;
  }

  FILE *fp = fopen(target_path, "w");
  if (fp == NULL) {
    printf("error: open file %s failed\n", target_path);
    return_result = OTHER_ERROR;
    goto dump_entry_exit_without_file_open;
  }

  char buf[BUFFER_SIZE];
  while (true) {
    int size = unzReadCurrentFile(file, buf, BUFFER_SIZE);
    if (size > 0) {
      if (fwrite(buf, size, 1, fp) != 1) {
        printf("error: write to file %s failed\n", target_path);
        return_result = OTHER_ERROR;
        goto dump_entry_exit;
      }
    } else if (size == 0) {
      break;
    } else {
      printf("error: read zip entry %s failed\n", filename);
      return_result = READ_ZIP_ENTRY_FAILED;
      goto dump_entry_exit;
    }
  }

  dump_entry_exit:
  if (fclose(fp) != 0) {
    printf("error: close file %s failed\n", target_path);
    return_result = OTHER_ERROR;
  }

  dump_entry_exit_without_file_open:
  if (unzCloseCurrentFile(file) != UNZ_OK) {
    printf("error: close zip entry %s failed\n", filename);
    return_result = CLOSE_ZIP_ENTRY_FAILED;
  }

  return return_result;
}

int unzip(const char *zip_path, const char *target_path) {
  int return_result = UNZIP_OK;
  unzFile file = unzOpen64(zip_path);
  if (file == NULL) {
    printf("error: open zip file %s failed\n", zip_path);
    return OPEN_ZIP_FILE_FAILED;
  }
  unz_global_info64 global_info;
  int err = unzGetGlobalInfo64(file, &global_info);
  if (err != UNZ_OK) {
    return_result = GET_ZIP_INFO_FAILED;
    goto unzip_exit;
  }
  char filename[MAX_FILENAME_LEN];
  char path_buf[MAX_FILENAME_LEN];
  int len = strlen(target_path);
  strncpy(path_buf, target_path, len);
  if (path_buf[len-1] != '/') {
    path_buf[len++] = '/';
  }
  for (int i = 0; i < global_info.number_entry; ++i) {
    memset(filename, 0, sizeof(filename));
    unz_file_info64 file_info;
    err = unzGetCurrentFileInfo64(file, &file_info, filename, sizeof(filename), NULL, 0, NULL, 0);
    if (err != UNZ_OK) {
      return_result = GET_FILE_INFO_FAILED;
      goto unzip_exit;
    }
    int l = (int)strlen(filename);
    strncpy(path_buf + len, filename, l);
    path_buf[len+l] = '\0';
    if (filename[l-1] == '/') {
      // this file is a directory
      err = mkdirs(path_buf);
      if (err != 0) {
        printf("error: create dir %s failed!!!\n", path_buf);
        return_result = err;
        goto unzip_exit;
      }
    } else {
      for (int i = len + l - 1; i > 0; --i) {
        if (path_buf[i] == '/') {
          path_buf[i] = '\0';
          err = mkdirs(path_buf);
          if (err != 0) {
            printf("error: create dir %s failed!!!\n", path_buf);
            return_result = err;
            goto unzip_exit;
          }
          path_buf[i] = '/';
          break;
        }
      }
      err = dump_entry(file, filename, path_buf);
      if (err != 0) {
        return_result = err;
        goto unzip_exit;
      }
    }

    err = unzGoToNextFile(file);
    if (err == UNZ_END_OF_LIST_OF_FILE) {
      break;
    } else if (err != UNZ_OK) {
      return_result = GO_TO_NEXT_FILE_FAILED;
      goto unzip_exit;
    }
  }

  unzip_exit:
  if (unzClose(file) != UNZ_OK) {
    return_result = CLOSE_ZIP_FILE_FAILED;
  }
  return return_result;
}

int create_dir(const char *dir) {
  #ifdef _WIN32
  return _mkdir(dir);
  #else
  return mkdir(dir, S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH);
  #endif
}

int mkdirs(const char *dir) {
  DIR* d = opendir(dir);
  if (d != NULL) {
    return closedir(d);
  }
  char buf[MAX_FILENAME_LEN];
  int len = (int)strlen(dir);
  if (len >= MAX_FILENAME_LEN) {
    printf("error: len of path %s exceed MAX_FILENAME_LEN %d\n", dir, MAX_FILENAME_LEN);
    return PATH_TOO_LONG;
  }
  strncpy(buf, dir, len);
  if (buf[len - 1] != '/') {
    buf[len++] = '/';
  }
  buf[len] = '\0';
  for (int i = 1; i < len; ++i) {
    if (buf[i] == '/') {
      buf[i] = '\0';
      d = opendir(buf);
      if (d == NULL) {
        if (create_dir(buf) != 0) {
          return CREATE_DIRECTORY_FAILED;
        }
      } else {
        closedir(d);
      }
      buf[i] = '/';
    }
  }
  return 0;
}

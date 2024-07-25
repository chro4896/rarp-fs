#define FUSE_USE_VERSION 32

#include <fuse3/fuse.h>
#include <string.h>
#include <errno.h>
#include <unistd.h> 
#include <sys/types.h>
#include <stdio.h>

int getattr_test_fuse (const char *path, struct stat *stbuf, struct fuse_file_info *fi) {
  memset(stbuf, 0, sizeof(struct stat));
  printf("getattr\n%u\n", sizeof(struct stat));

  if (strcmp(path, "/") == 0) {
    stbuf->st_mode = S_IFDIR | 0755;
    stbuf->st_nlink = 2;
    stbuf->st_uid = getuid();
    stbuf->st_gid = getgid();
    return 0;
  } else if (strcmp(path, "/test") == 0) {
    stbuf->st_mode = S_IFREG | 0777;
    stbuf->st_nlink = 1;
    stbuf->st_size = 1;
    stbuf->st_uid = getuid();
    stbuf->st_gid = getgid();
    return 0;
  }

  return -2;
}

int readdir_test_fuse (const char *path, void *buf, fuse_fill_dir_t filler, off_t offset, struct fuse_file_info *fi, enum fuse_readdir_flags flg) {

  filler(buf, ".", NULL, 0, flg);
  filler(buf, "..", NULL, 0, flg);
  filler(buf, "test", NULL, 0, flg);

  return 0;
}

static int open_test_fuse (const char *path, struct fuse_file_info *fi) {
  return 0;
}

static int read_test_fuse (const char *path, char *buf, size_t size, off_t offset, struct fuse_file_info *fi) {
  if (size < 1 || offset > 0) {
    return 0;
  }
  buf[0] = '\n';
  return 1;
}

static struct fuse_operations op = {
  .getattr = getattr_test_fuse,
  .open = open_test_fuse,
  .read = read_test_fuse,
};

int main (int argc, char *argv[]) {
  return fuse_main(argc, argv, &op, NULL);
}

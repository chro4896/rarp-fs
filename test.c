#include <fuse3/fuse.h>

#define FUSE_USE_VERSION 32

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
  .open = open_test_fuse,
  .read = read_test_fuse,
};

int main (int argc, char *argv[]) {
  return fuse_main(argc, argv, &op, NULL);
}

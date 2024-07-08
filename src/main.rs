#[link(name = "fuse3")]
extern {
	fn fuse_main_real(argc: core::ffi::c_int, argv: *mut *mut core::ffi::c_char, op: *const fuse_operations, op_size: usize, private_data: *mut core::ffi::c_void);
}

#[repr(C)]
struct fuse_operations {
}

fn main() {
    println!("Hello, world!");
}

pub extern "C" fn open_test_fuse(_path: *const core::ffi::c_char, _fi: *mut core::ffi::c_void) -> core::ffi::c_int {
	println!("open");
	0
}

pub extern "C" fn read_test_fuse(_path: *const core::ffi::c_char, buf: *mut core::ffi::c_char, size: usize, offset: libc::off_t, _fi: *mut core::ffi::c_void) -> core::ffi::c_int {
	println!("read");
	if size < 1 || offset > 0 {
        0
	} else {
		unsafe {
			*buf = 10;
		}
		1
	}
}

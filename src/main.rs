#[link(name = "fuse3")]
extern {
	fn fuse_main_real(argc: core::ffi::c_int, argv: *mut *mut core::ffi::c_char, op: *const fuse_operations, op_size: usize, private_data: *mut core::ffi::c_void);
}

#[repr(C)]
struct fuse_operations {
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let args: Vec<Box<core::ffi::CStr>> = args.iter().map(|s| {
		let mut v = s.into_bytes().to_vec();
		v.push(b'\0');
		Box::new(core::ffi::CStr::from_bytes_with_nul(v).unwrap())
	}).collect();
	let args: Vec<*mut core::ffi::c_char> = unsafe { args.iter().map(|s| s.as_ptr()).collect() };
	let fuse_op = fuse_operations {};
	unsafe { fuse_main_real(args.len(), args.as_mut_ptr(), &fuse_op, std::mem::size_of::<fuse_operations>(), std::ptr::null_mut()) };
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

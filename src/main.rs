static fuse_op: fuse_operations = fuse_operations {
		getattr: None,
		readlink: None,
		mknod: None,
		mkdir: None,
		unlink: None,
		rmdir: None,
		symlink: None,
		rename: None,
		link: None,
		chmod: None,
		chown: None,
		truncate: None,
		open: Some(open_test_fuse),
		read: Some(read_test_fuse),
	};

#[link(name = "fuse3")]
extern {
	fn fuse_main_real(argc: core::ffi::c_int, argv: *mut *mut core::ffi::c_char, op: *const fuse_operations, op_size: usize, private_data: *mut core::ffi::c_void);
}

#[repr(C)]
struct fuse_operations {
	getattr: Option<extern "C" fn(*const core::ffi::c_char, *mut core::ffi::c_void, *mut core::ffi::c_void) -> core::ffi::c_int>,
	readlink: Option<extern "C" fn(*const core::ffi::c_char, *mut core::ffi::c_char, usize) -> core::ffi::c_int>,
	mknod: Option<extern "C" fn(*const core::ffi::c_char, libc::mode_t, libc::dev_t) -> core::ffi::c_int>,
	mkdir: Option<extern "C" fn(*const core::ffi::c_char, libc::mode_t) -> core::ffi::c_int>,
	unlink: Option<extern "C" fn(*const core::ffi::c_char) -> core::ffi::c_int>,
	rmdir: Option<extern "C" fn(*const core::ffi::c_char) -> core::ffi::c_int>,
	symlink: Option<extern "C" fn(*const core::ffi::c_char, *const core::ffi::c_char) -> core::ffi::c_int>,
	rename: Option<extern "C" fn(*const core::ffi::c_char, *const core::ffi::c_char, *const core::ffi::c_uint) -> core::ffi::c_int>,
	link: Option<extern "C" fn(*const core::ffi::c_char, *const core::ffi::c_char) -> core::ffi::c_int>,
	chmod: Option<extern "C" fn(*const core::ffi::c_char, libc::mode_t, *const core::ffi::c_void) -> core::ffi::c_int>,
	chown: Option<extern "C" fn(*const core::ffi::c_char, libc::uid_t, libc::gid_t, *const core::ffi::c_void) -> core::ffi::c_int>,
	truncate: Option<extern "C" fn(*const core::ffi::c_char, libc::off_t, *const core::ffi::c_void) -> core::ffi::c_int>,
	open: Option<extern "C" fn(*const core::ffi::c_char, *mut core::ffi::c_void) -> core::ffi::c_int>,
	read: Option<extern "C" fn(*const core::ffi::c_char, *mut core::ffi::c_char, usize, libc::off_t, *mut core::ffi::c_void) -> core::ffi::c_int>,
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let mut args: Vec<Vec<core::ffi::c_char>> = args.into_iter().map(|s| {
		let mut v = s.into_bytes().to_vec();
		v.push(b'\0');
		v.iter().map(|c| *c as core::ffi::c_char).collect()
	}).collect();
	let mut args: Vec<*mut core::ffi::c_char> = args.iter_mut().map(|v| v.as_mut_ptr()).collect();
	unsafe { fuse_main_real(args.len().try_into().unwrap(), args.as_mut_ptr(), &fuse_op, std::mem::size_of::<fuse_operations>(), std::ptr::null_mut()) };
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

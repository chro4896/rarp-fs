#[link(name = "fuse3")]
extern {
	fn fuse_main_real(argc: core::ffi::c_int, argv: *mut *mut core::ffi::c_char, op: *const fuse_operations, op_size: usize, private_data: *mut core::ffi::c_void);
}

type fuse_fill_dir_t = extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_char, *const core::ffi::c_void, libc::off_t, i32) -> core::ffi::c_int;

#[repr(C)]
struct fuse_operations {
	open: *const core::ffi::c_void,
	read: *const core::ffi::c_void,
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let mut args: Vec<Vec<core::ffi::c_char>> = args.into_iter().map(|s| {
		let mut v = s.into_bytes().to_vec();
		v.push(b'\0');
		v.iter().map(|c| *c as core::ffi::c_char).collect()
	}).collect();
	let mut args: Vec<*mut core::ffi::c_char> = args.iter_mut().map(|v| v.as_mut_ptr()).collect();
	let fuse_op = fuse_operations {
		read: read_test_fuse as *const core::ffi::c_void,
		open: open_test_fuse as *const core::ffi::c_void,
	};
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

pub extern "C" fn readdir_test_fuse(_path: *const core::ffi::c_char, buf: *mut core::ffi::c_void, filler: fuse_fill_dir_t, _offset: libc::off_t, _fi: *mut core::ffi::c_void, flg: i32) -> core::ffi::c_int {
	filler(buf, [b'.' as core::ffi::c_char,b'\0' as core::ffi::c_char].as_ptr(), std::ptr::null(), 0, flg);
	filler(buf, [b'.' as core::ffi::c_char,b'.' as core::ffi::c_char,b'\0' as core::ffi::c_char].as_ptr(), std::ptr::null(), 0, flg);
	filler(buf, [b't' as core::ffi::c_char,b'e' as core::ffi::c_char,b's' as core::ffi::c_char,b't' as core::ffi::c_char,b'\0' as core::ffi::c_char].as_ptr(), std::ptr::null(), 0, flg);
	0
}

pub extern "C" fn getattr_test_fuse(path: *const core::ffi::c_char, stbuf: *mut libc::stat, _fi: *mut core::ffi::c_void) -> core::ffi::c_int {
	unsafe { libc::memset(stbuf as *mut libc::c_void, 0, std::mem::size_of::<libc::stat>()) };
	let stbuf = &mut unsafe { *stbuf };
	stbuf.st_uid = unsafe { libc::getuid() };
	stbuf.st_gid = unsafe { libc::getgid() };
	if unsafe { *(path.offset(1)) } == b'\0' as core::ffi::c_char {
		stbuf.st_mode = libc::S_IFDIR | 0o775;
		stbuf.st_nlink = 2;
	} else {
		stbuf.st_mode = libc::S_IFREG | 0o664;
		stbuf.st_nlink = 1;
		stbuf.st_size = 1;
	}
	0
}

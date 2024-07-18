static fuse_op: fuse_operations = fuse_operations {
		getattr: Some(getattr_test_fuse),
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

type fuse_fill_dir_t = extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_char, *const core::ffi::c_void, libc::off_t, i32) -> core::ffi::c_int;

#[repr(C)]
struct fuse_operations {
	getattr: Option<extern "C" fn(*const core::ffi::c_char, *mut libc::stat, *mut core::ffi::c_void) -> core::ffi::c_int>,
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

pub extern "C" fn readdir_test_fuse(_path: *const core::ffi::c_char, buf: *mut core::ffi::c_void, filler: fuse_fill_dir_t, _offset: libc::off_t, _fi: *mut core::ffi::c_void, flg: i32) -> core::ffi::c_int {
	filler(buf, [b'.' as core::ffi::c_char,b'\0' as core::ffi::c_char].as_ptr(), std::ptr::null(), 0, flg);
	filler(buf, [b'.' as core::ffi::c_char,b'.' as core::ffi::c_char,b'\0' as core::ffi::c_char].as_ptr(), std::ptr::null(), 0, flg);
	filler(buf, [b't' as core::ffi::c_char,b'e' as core::ffi::c_char,b's' as core::ffi::c_char,b't' as core::ffi::c_char,b'\0' as core::ffi::c_char].as_ptr(), std::ptr::null(), 0, flg);
	0
}

pub extern "C" fn getattr_test_fuse(path: *const core::ffi::c_char, stbuf: *mut libc::stat, _fi: *mut core::ffi::c_void) -> core::ffi::c_int {
	println!("getattr");
	unsafe { libc::memset(stbuf as *mut libc::c_void, 0, std::mem::size_of::<libc::stat>()) };
	let stbuf = &mut unsafe { *stbuf };
	stbuf.st_uid = unsafe { libc::getuid() };
	stbuf.st_gid = unsafe { libc::getgid() };
	if unsafe { *(path.offset(1)) } == b'\0' as core::ffi::c_char {
		stbuf.st_mode = libc::S_IFDIR | 0o775;
		stbuf.st_nlink = 2;
		0
	} else if unsafe { *(path.offset(1)) as u8 } == b't' && unsafe { *(path.offset(2)) as u8 } == b'e' && unsafe { *(path.offset(3)) as u8 } == b's' && unsafe { *(path.offset(4)) as u8 } == b't' {
		stbuf.st_mode = libc::S_IFREG | 0o664;
		stbuf.st_nlink = 1;
		stbuf.st_size = 1;
		0
	} else {
		(-1)*libc::ENOENT
	}
}

#[link(name = "fuse3")]
extern {
	fn fuse_main_real(argc: core::ffi::c_int, argv: *mut *mut core::ffi::c_char, op: *const fuse_operations, op_size: usize, private_data: *mut core::ffi::c_void);
}

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
	write: *const core::ffi::c_void,
	statfs: *const core::ffi::c_void,
	flush: *const core::ffi::c_void,
	release: *const core::ffi::c_void,
	fsync: *const core::ffi::c_void,
	setxattr: *const core::ffi::c_void,
	getxattr: *const core::ffi::c_void,
	listxattr: *const core::ffi::c_void,
	removexattr: *const core::ffi::c_void,
	opendir: *const core::ffi::c_void,
	readdir: *const core::ffi::c_void,
	releasedir: *const core::ffi::c_void,
	fsyncdir: *const core::ffi::c_void,
	init: *const core::ffi::c_void,
	destroy: *const core::ffi::c_void,
	access: *const core::ffi::c_void,
	create: *const core::ffi::c_void,
	lock: *const core::ffi::c_void,
	utimens: *const core::ffi::c_void,
	bmap: *const core::ffi::c_void,
	ioctl: *const core::ffi::c_void,
	poll: *const core::ffi::c_void,
	write_buf: *const core::ffi::c_void,
	read_buf: *const core::ffi::c_void,
	flock: *const core::ffi::c_void,
	fallocate: *const core::ffi::c_void,
	copy_file_range: *const core::ffi::c_void,
	lseek: *const core::ffi::c_void,
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
		write: std::ptr::null(),
		statfs: std::ptr::null(),
		flush: std::ptr::null(),
		release: std::ptr::null(),
		fsync: std::ptr::null(),
		setxattr: std::ptr::null(),
		getxattr: std::ptr::null(),
		listxattr: std::ptr::null(),
		removexattr: std::ptr::null(),
		opendir: std::ptr::null(),
		readdir: std::ptr::null(),
		releasedir: std::ptr::null(),
		fsyncdir: std::ptr::null(),
		init: std::ptr::null(),
		destroy: std::ptr::null(),
		access: std::ptr::null(),
		create: std::ptr::null(),
		lock: std::ptr::null(),
		utimens: std::ptr::null(),
		bmap: std::ptr::null(),
		ioctl: std::ptr::null(),
		poll: std::ptr::null(),
		write_buf: std::ptr::null(),
		read_buf: std::ptr::null(),
		flock: std::ptr::null(),
		fallocate: std::ptr::null(),
		copy_file_range: std::ptr::null(),
		lseek: std::ptr::null(),
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

pub extern "C" fn getattr_test_fuse(path: *const core::ffi::c_char, stbuf: *mut libc::stat, _fi: *mut core::ffi::c_void) -> core::ffi::c_int {
		(-1)*libc::ENOENT
}

pub extern "C" fn getattr_test_fuse_(path: *const core::ffi::c_char, stbuf: *mut libc::stat, _fi: *mut core::ffi::c_void) -> core::ffi::c_int {
	println!("getattr");
	println!("{}", std::mem::size_of::<libc::stat>());
	unsafe { libc::memset(stbuf as *mut libc::c_void, 0, std::mem::size_of::<libc::stat>()) };
	let stbuf = &mut unsafe { *stbuf };
	stbuf.st_uid = unsafe { libc::getuid() };
	stbuf.st_gid = unsafe { libc::getgid() };
	if unsafe { *(path.offset(1)) } == b'\0' as core::ffi::c_char {
		stbuf.st_mode = libc::S_IFDIR | 0o775;
		stbuf.st_nlink = 2;
		println!("{:b}", stbuf.st_mode);
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

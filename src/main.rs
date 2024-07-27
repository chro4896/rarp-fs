#[link(name = "fuse3")]
extern {
	fn fuse_main_real(argc: core::ffi::c_int, argv: *mut *mut core::ffi::c_char, op: *const fuse_operations, op_size: usize, private_data: *mut core::ffi::c_void);
}

type fuse_fill_dir_t = extern "C" fn(*mut core::ffi::c_void, *const core::ffi::c_char, *const core::ffi::c_void, libc::off_t, i32) -> core::ffi::c_int;

#[repr(C)]
struct fuse_operations {
	getattr: *const core::ffi::c_void,
	readlink: *const core::ffi::c_void,
	mknod: *const core::ffi::c_void,
	mkdir: *const core::ffi::c_void,
	unlink: *const core::ffi::c_void,
	rmdir: *const core::ffi::c_void,
	symlink: *const core::ffi::c_void,
	rename: *const core::ffi::c_void,
	link: *const core::ffi::c_void,
	chmod: *const core::ffi::c_void,
	chown: *const core::ffi::c_void,
	truncate: *const core::ffi::c_void,
	open: *const core::ffi::c_void,
	read: *const core::ffi::c_void,
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
		getattr: getattr_test_fuse as *const core::ffi::c_void,
		readlink: std::ptr::null(),
		mknod: std::ptr::null(),
		mkdir: std::ptr::null(),
		unlink: std::ptr::null(),
		rmdir: std::ptr::null(),
		symlink: std::ptr::null(),
		rename: std::ptr::null(),
		link: std::ptr::null(),
		chmod: std::ptr::null(),
		chown: std::ptr::null(),
		truncate: std::ptr::null(),
		open: open_test_fuse as *const core::ffi::c_void,
		read: read_test_fuse as *const core::ffi::c_void,
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

pub extern "C" fn readdir_test_fuse(_path: *const core::ffi::c_char, buf: *mut core::ffi::c_void, filler: fuse_fill_dir_t, _offset: libc::off_t, _fi: *mut core::ffi::c_void, flg: i32) -> core::ffi::c_int {
	filler(buf, [b'.' as core::ffi::c_char,b'\0' as core::ffi::c_char].as_ptr(), std::ptr::null(), 0, flg);
	filler(buf, [b'.' as core::ffi::c_char,b'.' as core::ffi::c_char,b'\0' as core::ffi::c_char].as_ptr(), std::ptr::null(), 0, flg);
	filler(buf, [b't' as core::ffi::c_char,b'e' as core::ffi::c_char,b's' as core::ffi::c_char,b't' as core::ffi::c_char,b'\0' as core::ffi::c_char].as_ptr(), std::ptr::null(), 0, flg);
	0
}

pub extern "C" fn getattr_test_fuse(path: *const core::ffi::c_char, stbuf: Option<&mut libc::stat>, _fi: *mut core::ffi::c_void) -> core::ffi::c_int {
	let stbuf = stbuf.unwrap();
	println!("getattr");
	unsafe { libc::memset(stbuf as *mut libc::stat as *mut libc::c_void, 0, std::mem::size_of::<libc::stat>()) };
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

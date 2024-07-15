#[link(name = "fuse3")]
extern {
	fn fuse_main_real(argc: core::ffi::c_int, argv: *mut *mut core::ffi::c_char, op: *const fuse_operations, op_size: usize, private_data: *mut core::ffi::c_void);
}

#[repr(C)]
struct fuse_operations {
	open: extern "C" fn(*const core::ffi::c_char, *mut core::ffi::c_void) -> core::ffi::c_int,
	read: extern "C" fn(*const core::ffi::c_char, *mut core::ffi::c_char, usize, libc::off_t, *mut core::ffi::c_void) -> core::ffi::c_int,
}

trait FuseOperations {
	type OPEN_FN: Fn(*const core::ffi::c_char, *mut core::ffi::c_void) -> core::ffi::c_int;
	type READ_FN: Fn(*const core::ffi::c_char, *mut core::ffi::c_char, usize, libc::off_t, *mut core::ffi::c_void) -> core::ffi::c_int;
	fn get_open_fn (&self) -> Option<OPEN_FN>;
	fn get_read_fn (&self) -> Option<READ_FN>;
}

struct fuse_operations_rust<OPEN_FN=fn(*const core::ffi::c_char, *mut core::ffi::c_void) -> core::ffi::c_int, READ_FN=fn(*const core::ffi::c_char, *mut core::ffi::c_char, usize, libc::off_t, *mut core::ffi::c_void) -> core::ffi::c_int> {
	open: Option<OPEN_FN>,
	read: Option<READ_FN>, 
}

impl<OPEN_FN, READ_FN> FuseOperations for fuse_operations_rust<OPEN_FN, READ_FN> where
    OPEN_FN: Fn(*const core::ffi::c_char, *mut core::ffi::c_void) -> core::ffi::c_int,
    READ_FN: Fn(*const core::ffi::c_char, *mut core::ffi::c_char, usize, libc::off_t, *mut core::ffi::c_void) -> core::ffi::c_int,
{
	type OPEN_FN = OPEN_FN;
	type READ_FN = READ_FN;
	fn get_open_fn (&self) -> Option<OPEN_FN> {
		self.open
	}
	fn get_read_fn (&self) -> Option<READ_FN> {
		self.read
	}
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
		open: open_test_fuse,
		read: read_test_fuse,
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

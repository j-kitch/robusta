use std::io::{stderr, stdout, Write};
use std::slice::from_raw_parts;
use std::sync::Arc;
use crate::java::{FieldType, MethodType, Value};
use crate::method_area::const_pool::FieldKey;
use crate::native::{Args, Plugin};
use crate::native::stateless::{Method, stateless};

pub fn file_output_stream_plugins() -> Vec<Arc<dyn Plugin>> {
    vec![
        stateless(
            Method {
                class: "java.io.FileOutputStream".to_string(),
                name: "writeBytes".to_string(),
                descriptor: MethodType::from_descriptor("([BIIZ)V").unwrap(),
            },
            Arc::new(write_bytes),
        ),
    ]
}

fn write_bytes(args: &Args) -> (Option<Value>, Option<Value>) {
    let fos_ref = args.params[0].reference();
    let bytes_ref = args.params[1].reference();
    let off = args.params[2].int().0 as usize;
    let len = args.params[3].int().0 as usize;
    let _ = args.params[4].int().0 != 0;

    let fos_obj = args.runtime.heap.get_object(fos_ref);

    let fd_ref = fos_obj.get_field(&FieldKey {
        class: "java.io.FileOutputStream".to_string(),
        name: "fd".to_string(),
        descriptor: FieldType::from_descriptor("Ljava/io/FileDescriptor;").unwrap(),
    }).reference();
    let fd_obj = args.runtime.heap.get_object(fd_ref);

    let fd = fd_obj.get_field(&FieldKey {
        class: "java.io.FileDescriptor".to_string(),
        name: "fd".to_string(),
        descriptor: FieldType::Int,
    }).int().0;

    let bytes = args.runtime.heap.get_array(bytes_ref);
    let bytes = bytes.as_bytes_slice();

    let unsigned_bytes = unsafe {
        let ptr = bytes.as_ptr();
        let ptr: *const u8 = ptr.cast();
        from_raw_parts(ptr, bytes.len())
    };

    if fd == 1 {
        stdout().write_all(&unsigned_bytes[off..(off+len)]).unwrap();
    } else if fd == 2 {
        stderr().write_all(&unsigned_bytes[off..(off+len)]).unwrap();
    } else {
        panic!("Not implemented generic FileOutputStream yet!")
    }

    (None, None)
}
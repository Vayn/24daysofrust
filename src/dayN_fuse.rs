extern crate fuse;
extern crate time;
extern crate libc;
extern crate serialize;

use std::io::{FileType, USER_FILE, USER_DIR};
use std::mem;
use std::os;
use libc::{ENOENT, ENOSYS};
use time::Timespec;
use fuse::{FileAttr, Filesystem, Request, ReplyAttr, ReplyEntry, ReplyDirectory};
use serialize::json;

struct JsonFilesystem {
    tree: json::Object,
}

impl Filesystem for JsonFilesystem {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(): ino {}", ino);
        let mut attr: FileAttr = unsafe { mem::zeroed() };
        attr.ino = 1;
        attr.kind = FileType::Directory;
        attr.perm = USER_DIR;
        let ttl = Timespec::new(1, 0);
        if ino == 1 {
            reply.attr(&ttl, &attr);
        } else {
            reply.error(ENOSYS);
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: u64, mut reply: ReplyDirectory) {
        println!("readdir(): ino {}, fh {}, ofset {}", ino, fh, offset);
        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, &PosixPath::new("."));
                reply.add(1, 1, FileType::Directory, &PosixPath::new(".."));
                for (i, key) in self.tree.keys().enumerate() {
                    let inode: u64 = 2 + i as u64;
                    let offset: u64 = 2 + i as u64;
                    reply.add(inode, offset, FileType::RegularFile, &PosixPath::new(key));
                }
            }
            reply.ok();
        } else {
            reply.error(ENOENT);
        }
    }
}

fn main() {
    println!("24 days of Rust - fuse (day N)");
    let data = json::from_str("{\"foo\": \"bar\", \"answer\": 42}").unwrap();
    let tree = data.as_object().unwrap();
    let fs = JsonFilesystem { tree: tree.clone() };
    let mountpoint = Path::new(os::args()[1].as_slice());
    fuse::mount(fs, &mountpoint, &[]);
}
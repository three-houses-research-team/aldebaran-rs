use ktgl::feth::v120::memory::kt_aligned_malloc;
use lazy_static::lazy_static;
use skyline::alloc::slice;
use skyline::libc::c_void;
use skyline::{hook, install_hooks};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::{fs, io};

lazy_static! {
    pub static ref FORGE: Forge = Forge::new();
}

pub struct Forge {
    cache: HashMap<u32, String>,
}

impl Forge {
    fn new() -> Forge {
        let mut instance = Forge {
            cache: HashMap::new(),
        };

        instance.visit_dirs(Path::new("sd:/Aldebaran/forge")).ok();

        install_hooks!(hook_load_with_entry_id, hook_get_uncompressed_size);

        instance
    }

    fn visit_file(&mut self, real_path: String, filename: String) {
        let index_options = vec![
            filename.parse::<u32>().ok(),
            filename
                .split("-")
                .nth(0)
                .and_then(|x| x.parse::<u32>().ok()),
        ];
        if let Some(index) = index_options.iter().filter_map(|&x| x).nth(0) {
            println!("[Forge] Discovered {} => {}", index, real_path);
            self.cache.insert(index, real_path);
        }
    }

    fn visit_dirs(&mut self, dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let filename = entry.path(); // this looks wrong but is actually right
                let real_path = format!("{}/{}", dir.to_str().unwrap(), filename.to_str().unwrap());
                let path = Path::new(&real_path);
                if path.is_dir() {
                    self.visit_dirs(&path)?;
                } else {
                    self.visit_file(real_path, filename.to_str().unwrap().to_string());
                }
            }
        }
        Ok(())
    }

    fn get_path_from_index(&self, index: u32) -> Option<&String> {
        self.cache.get(&index)
    }

    pub fn get_filesize_for_fileid(&self, fileid: u32) -> Option<u64> {
        let cached_path = match self.get_path_from_index(fileid) {
            None => {
                return None;
            }
            Some(path) => path,
        };

        let file = File::open(cached_path).unwrap();
        let metadata = file.metadata().unwrap();

        if metadata.is_dir() {
            return None;
        }

        Some(metadata.len())
    }

    fn try_load(
        &self,
        entryid: u32,
        seek: u32,
        size: u64,
        file_ptr: *const c_void,
    ) -> Option<*const c_void> {
        let cached_path = self.get_path_from_index(entryid)?;
        println!("[Forge] EntryID {} loaded from {}.", entryid, cached_path);
        let mut file = File::open(cached_path).ok()?;
        let metadata = file.metadata().ok()?;

        if metadata.is_dir() {
            return None;
        }

        file.seek(SeekFrom::Start(seek as u64)).ok()?;
        let len: u64 = match size {
            0 => metadata.len(),
            requested => requested,
        };

        unsafe {
            let free_mem: *const c_void;
            if file_ptr != 0 as _ {
                free_mem = file_ptr;
            } else {
                free_mem = kt_aligned_malloc(len, 0x10);
            }
            let content = slice::from_raw_parts_mut(free_mem as *mut u8, len as usize);
            file.read_exact(content).ok()?;
            Some(content.as_ptr() as *const c_void)
        }
    }
}

#[hook(offset = 0x4A12B0)]
pub fn hook_load_with_entry_id(
    archive_ptr: *const c_void,
    entryid: u32,
    file_ptr: *const c_void,
    seek: u32,
    size: u64,
    unk1: u64,
    unk2: u64,
    unk3: u64,
) -> *const c_void {
    match FORGE.try_load(entryid, seek, size, file_ptr) {
        None => {
            println!("[Forge] EntryID {} loaded.", entryid);
            original!()(archive_ptr, entryid, file_ptr, seek, size, unk1, unk2, unk3)
        }
        Some(ptr) => ptr, // log already handled by implementation
    }
}

#[hook(offset = 0x4A0B40)]
pub fn hook_get_uncompressed_size(linkdata_mgr: *const c_void, file_id: u32) -> u64 {
    let filesize = match FORGE.get_filesize_for_fileid(file_id) {
        None => return original!()(linkdata_mgr, file_id),
        Some(size) => size,
    };
    println!(
        "[Forge] Size of FileID {} requested, replying with {:#x}",
        file_id, filesize
    );
    filesize
}

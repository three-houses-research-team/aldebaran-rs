use ktgl::feth::v120::linkdata::{ get_uncompressed_size };
use ktgl::feth::v120::memory::kt_aligned_malloc;
use lazy_static::lazy_static;
use skyline::alloc::slice;
use skyline::libc;
use skyline::libc::c_void;
use skyline::{hook, install_hook};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::Mutex;
use std::{fs, io};

lazy_static! {
    static ref FORGE: Mutex<Forge> = Mutex::new(Forge::new());
}

macro_rules! forge {
    () => {
        FORGE.lock().unwrap()
    };
}

struct Forge {
    cache: HashMap<u32, String>,
}

impl Forge {
    fn get_path_from_index(&self, index: u32) -> Option<&String> {
        self.cache.get(&index)
    }

    fn new() -> Forge {
        Forge {
            cache: HashMap::new(),
        }
    }

    fn try_load(&self, entryid: u32, seek: u32, size: u64) -> Option<*const c_void> {
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
            let free_mem = kt_aligned_malloc(len, 0x10);
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
    match forge!().try_load(entryid, seek, size) {
        None => {
            println!("[Forge] EntryID {} loaded.", entryid);
            original!()(archive_ptr, entryid, file_ptr, seek, size, unk1, unk2, unk3)
        }
        Some(ptr) => ptr, // log already handled by implementation
    }
}

#[hook(replace = get_uncompressed_size)]
pub fn hook_get_uncompressed_size(linkdata_mgr: *const c_void, file_id: u32) -> u64 {
    let forge_instance = forge!();
    let cached_path = match forge_instance.get_path_from_index(file_id) {
        None => {
            return original!()(linkdata_mgr, file_id);
        }
        Some(filepath) => filepath,
    };

    let file = File::open(cached_path).ok().unwrap();
    let metadata = file.metadata().ok().unwrap();

    if metadata.is_dir() {
        return original!()(linkdata_mgr, file_id);
    }
    let filesize = metadata.len();
    println!("[Forge] Size of FileID {} requested, replying with {:x}", file_id, filesize);
    filesize

    
}

pub fn init_forge() {
    install_hook!(hook_load_with_entry_id);
    install_hook!(hook_get_uncompressed_size);

    // we write a custom implementation of a directory tree walker because the implementation does not behave like spec
    // which will make walkdir and friends fail

    fn visit_file(real_path: String, filename: String) {
        let index_options = vec![
            filename.parse::<u32>().ok(),
            filename
                .split("-")
                .nth(0)
                .and_then(|x| x.parse::<u32>().ok()),
        ];
        if let Some(index) = index_options.iter().filter_map(|&x| x).nth(0) {
            println!("[Forge] Discovered {} => {}", index, real_path);
            forge!().cache.insert(index, real_path);
        }
    }

    fn visit_dirs(dir: &Path) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let filename = entry.path(); // this looks wrong but is actually right
                let real_path = format!("{}/{}", dir.to_str().unwrap(), filename.to_str().unwrap());
                let path = Path::new(&real_path);
                if path.is_dir() {
                    visit_dirs(&path)?;
                } else {
                    visit_file(real_path, filename.to_str().unwrap().to_string());
                }
            }
        }
        Ok(())
    }
    visit_dirs(Path::new("sd:/Aldebaran/forge")).ok();
}

use clap::Clap;
use std::path;
use std::fs;
use std::io;
use std::ffi;

#[cfg(target_family = "unix")]
use std::os::unix::fs::symlink as soft_link;

#[cfg(target_os = "windows")]
use std::os::windows::fs::{symlink_file, symlink_dir};

#[cfg(target_os = "windows")]
fn soft_link(src: &path::Path, dst: &path::Path) -> io::Result<()>{
    if src.is_dir() {
        return symlink_dir(src, dst)
    } else {
        return symlink_file(src, dst)
    }
}

#[derive(Clap, Debug)]
struct Args {
    #[clap(short, long, about = "Insert symbolic links recursively")]
    recursive: bool,
    #[clap(about = "Directory from where to take symbolic links")]
    src_dir: String,
    #[clap(about = "Directory where to insert symbolic links")]
    dst_dir: String,
}
type ListTup = (ffi::OsString, path::PathBuf);
type VecList = Vec<ListTup>;


fn listDir(p: &path::Path) -> io::Result<VecList> {
    let mut vec = VecList::new();
    for entry in fs::read_dir(p)? {
        let e = entry?.path();
        vec.push((e.file_name().unwrap().to_os_string(), e))
    }
    Ok(vec)
}

fn findInListing<'a>(fname: &ffi::OsString, list: &'a VecList) -> Option<&'a ListTup> {
    for tf in list {
        if &tf.0 == fname {
            return Some(tf)
        }
    }
    None
}

fn inListing(fname: &ffi::OsString, list: &VecList) -> bool {
    match findInListing(fname, list) {
        Some(_) => true,
        _ => false
    }
}

fn mergeDirs(src: &path::Path, dst: &path::Path, recursive: bool) 
    -> io::Result<()> 
{
    let src_list = listDir(src)?;
    let dst_list = listDir(dst)?;
    if recursive {
        for (dfname, dfpath) in &dst_list {
            if !dfpath.is_dir() {continue}
            if let Some((_, sfpath)) = findInListing(&dfname, &src_list) {
                mergeDirs(sfpath, dfpath, recursive)?;
            }
        }
    }

    for (fname, fpath) in src_list {
        if !inListing(&fname, &dst_list) {
            soft_link(&fpath, &dst.join(&fname))?;
        }
    }

    Ok(())
}


fn main() {
    let args = Args::parse();
    #[cfg(debug_assertions)]
    println!("{:?}", args);

    let src = path::Path::new(&args.src_dir);
    if !src.is_dir() {
        println!("Error: src_dir must be an existing directory");
        return ()
    }

    let dst = path::Path::new(&args.dst_dir);
    if !dst.is_dir() {
        println!("Error: dst_dir must be an existing directory");
        return ()
    }

    mergeDirs(&src, &dst, args.recursive).unwrap();
}

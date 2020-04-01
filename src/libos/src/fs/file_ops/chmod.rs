use super::*;

bitflags! {
    pub struct FileMode: u16 {
        /// set-user-ID
        const S_ISUID = 0o4000;
        /// set-group-ID
        const S_ISGID = 0o2000;
        /// sticky bit
        const S_ISVTX = 0o1000;
        /// read by owner
        const S_IRUSR = 0o0400;
        /// write by owner
        const S_IWUSR = 0o0200;
        /// execute/search by owner
        const S_IXUSR = 0o0100;
        /// read by group
        const S_IRGRP = 0o0040;
        /// write by group
        const S_IWGRP = 0o0020;
        /// execute/search by group
        const S_IXGRP = 0o0010;
        /// read by others
        const S_IROTH = 0o0004;
        /// write by others
        const S_IWOTH = 0o0002;
        /// execute/search by others
        const S_IXOTH = 0o0001;
    }
}

impl FileMode {
    pub fn is_readable(&self) -> bool {
        self.contains(FileMode::S_IRUSR)
    }

    pub fn is_writable(&self) -> bool {
        self.contains(FileMode::S_IWUSR)
    }

    pub fn is_executable(&self) -> bool {
        self.contains(FileMode::S_IXUSR)
    }

    pub fn has_sticky_bit(&self) -> bool {
        self.contains(FileMode::S_ISVTX)
    }

    pub fn has_set_uid(&self) -> bool {
        self.contains(FileMode::S_ISUID)
    }

    pub fn has_set_gid(&self) -> bool {
        self.contains(FileMode::S_ISGID)
    }
}

pub fn do_chmod(path: &str, mode: FileMode) -> Result<()> {
    debug!("chmod: path: {:?}, mode: {:?}", path, mode);
    let inode = {
        let current_ref = process::get_current();
        let mut current = current_ref.lock().unwrap();
        current.lookup_inode(path)?
    };
    let mut info = inode.metadata()?;
    info.mode = mode.bits();
    inode.set_metadata(&info)?;
    Ok(())
}

pub fn do_fchmod(fd: FileDesc, mode: FileMode) -> Result<()> {
    debug!("fchmod: fd: {}, mode: {:?}", fd, mode);
    let file_ref = process::get_file(fd)?;
    let mut info = file_ref.metadata()?;
    info.mode = mode.bits();
    file_ref.set_metadata(&info)?;
    Ok(())
}

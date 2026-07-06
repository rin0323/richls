use std::ffi::CStr;
use std::fs::Metadata;
use std::os::raw::{c_char, c_int};
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(unix)]
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};

#[repr(C)]
struct Passwd {
    pw_name: *mut c_char,
    pw_passwd: *mut c_char,
    pw_uid: u32,
    pw_gid: u32,
    pw_gecos: *mut c_char,
    pw_dir: *mut c_char,
    pw_shell: *mut c_char,
}

#[repr(C)]
struct Group {
    gr_name: *mut c_char,
    gr_passwd: *mut c_char,
    gr_gid: u32,
    gr_mem: *mut *mut c_char,
}

#[repr(C)]
struct Tm {
    tm_sec: c_int,
    tm_min: c_int,
    tm_hour: c_int,
    tm_mday: c_int,
    tm_mon: c_int,
    tm_year: c_int,
    tm_wday: c_int,
    tm_yday: c_int,
    tm_isdst: c_int,
    tm_gmtoff: isize,
    tm_zone: *const c_char,
}

unsafe extern "C" {
    fn getpwuid(uid: u32) -> *mut Passwd;
    fn getgrgid(gid: u32) -> *mut Group;
    fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
}

pub fn mode_string(metadata: &Metadata) -> String {
    #[cfg(unix)]
    {
        let mode = metadata.permissions().mode();
        let mut output = String::with_capacity(10);
        output.push(file_type_char(metadata));

        for bit in [
            0o400, 0o200, 0o100, 0o040, 0o020, 0o010, 0o004, 0o002, 0o001,
        ] {
            output.push(match (mode & bit != 0, bit) {
                (true, 0o400 | 0o040 | 0o004) => 'r',
                (true, 0o200 | 0o020 | 0o002) => 'w',
                (true, 0o100 | 0o010 | 0o001) => 'x',
                _ => '-',
            });
        }

        output
    }

    #[cfg(not(unix))]
    {
        if metadata.is_dir() {
            "drwxr-xr-x".to_string()
        } else {
            "-rw-r--r--".to_string()
        }
    }
}

#[cfg(unix)]
fn file_type_char(metadata: &Metadata) -> char {
    let file_type = metadata.file_type();
    if file_type.is_dir() {
        'd'
    } else if file_type.is_symlink() {
        'l'
    } else if file_type.is_socket() {
        's'
    } else if file_type.is_fifo() {
        'p'
    } else if file_type.is_char_device() {
        'c'
    } else if file_type.is_block_device() {
        'b'
    } else {
        '-'
    }
}

pub fn link_count(metadata: &Metadata) -> u64 {
    #[cfg(unix)]
    {
        metadata.nlink()
    }

    #[cfg(not(unix))]
    {
        let _ = metadata;
        1
    }
}

pub fn owner_name(metadata: &Metadata) -> String {
    #[cfg(unix)]
    {
        let uid = metadata.uid();
        unsafe {
            let passwd = getpwuid(uid);
            if passwd.is_null() || (*passwd).pw_name.is_null() {
                return uid.to_string();
            }
            CStr::from_ptr((*passwd).pw_name)
                .to_string_lossy()
                .into_owned()
        }
    }

    #[cfg(not(unix))]
    {
        let _ = metadata;
        "-".to_string()
    }
}

pub fn group_name(metadata: &Metadata) -> String {
    #[cfg(unix)]
    {
        let gid = metadata.gid();
        unsafe {
            let group = getgrgid(gid);
            if group.is_null() || (*group).gr_name.is_null() {
                return gid.to_string();
            }
            CStr::from_ptr((*group).gr_name)
                .to_string_lossy()
                .into_owned()
        }
    }

    #[cfg(not(unix))]
    {
        let _ = metadata;
        "-".to_string()
    }
}

pub fn human_size(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut value = size as f64;
    let mut unit = 0;

    while value >= 1024.0 && unit < units.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{size}B")
    } else {
        format!("{value:.1}{}", units[unit])
    }
}

pub fn format_system_time(time: SystemTime) -> String {
    let Ok(duration) = time.duration_since(UNIX_EPOCH) else {
        return "-".to_string();
    };
    let timestamp = duration.as_secs() as i64;

    unsafe {
        let mut tm = std::mem::zeroed();
        if localtime_r(&timestamp, &mut tm).is_null() {
            return "-".to_string();
        }

        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}",
            tm.tm_year + 1900,
            tm.tm_mon + 1,
            tm.tm_mday,
            tm.tm_hour,
            tm.tm_min
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn humanizes_sizes() {
        assert_eq!(human_size(0), "0B");
        assert_eq!(human_size(999), "999B");
        assert_eq!(human_size(1023), "1023B");
        assert_eq!(human_size(1024), "1.0KB");
        assert_eq!(human_size(1536), "1.5KB");
        assert_eq!(human_size(1_288_490_188), "1.2GB");
    }

    #[test]
    fn formats_times_for_humans() {
        let formatted = format_system_time(UNIX_EPOCH);

        assert_eq!(formatted.len(), 16);
        assert_eq!(&formatted[4..5], "-");
        assert_eq!(&formatted[7..8], "-");
        assert_eq!(&formatted[10..11], " ");
        assert_eq!(&formatted[13..14], ":");
    }

    #[test]
    fn rejects_times_before_the_unix_epoch() {
        let before_epoch = UNIX_EPOCH - std::time::Duration::from_secs(1);
        assert_eq!(format_system_time(before_epoch), "-");
    }
}

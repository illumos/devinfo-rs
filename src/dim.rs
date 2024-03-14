/*
 * Copyright 2024 Oxide Computer Company
 */

use anyhow::{bail, Result};
use libc::{c_void, free};
use libdevinfo_sys::*;
use std::ffi::OsStr;
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStrExt;

pub struct DevInstMinor {
    handle: *mut di_dim_t,
}

impl DevInstMinor {
    pub fn new() -> Result<Self> {
        let handle = unsafe { di_dim_init() };
        if handle.is_null() {
            let e = std::io::Error::last_os_error();
            bail!("di_dim_init: {}", e);
        }

        Ok(DevInstMinor { handle })
    }

    /**
     * Given a driver (e.g., "blkdev"), an instance (e.g., 0), and a minor
     * (e.g., "wd"), look up the primary dev path for the device; e.g.,
     * "/dev/dsk/c1t0025385C9150D623d0".  This is the same logic that underpins
     * tools like iostat(8) when used with the "-n" flag to translate instances
     * of drivers into public disk names.
     */
    pub fn lookup_dev(
        &self,
        driver: &str,
        instance: u32,
        minor: &str,
    ) -> Option<String> {
        let cdrv = CString::new(driver).ok()?;
        let cmin = CString::new(minor).ok()?;

        let res = unsafe {
            di_dim_path_dev(
                self.handle,
                cdrv.as_ptr(),
                instance.try_into().unwrap(),
                cmin.as_ptr(),
            )
        };
        if res.is_null() {
            None
        } else {
            let restr = unsafe { CStr::from_ptr(res) };
            let out = restr.to_str().ok().map(str::to_string);
            unsafe { free(res as *mut c_void) };
            out
        }
    }

    /**
     * Given a driver (e.g., "blkdev"), and an instance (e.g., 0) look up the
     * public name for a disk device; e.g., "c1t0025385C9150D623d0".  This is
     * the same logic that underpins tools like iostat(8) when used with the
     * "-n" flag to translate instances of drivers into public disk names.
     */
    pub fn lookup_disk_name(
        &self,
        driver: &str,
        instance: u32,
    ) -> Option<String> {
        let cdrv = CString::new(driver).ok()?;
        for minor in ["wd", "a"] {
            let cmin = CString::new(minor).ok()?;

            let res = unsafe {
                di_dim_path_dev(
                    self.handle,
                    cdrv.as_ptr(),
                    instance.try_into().unwrap(),
                    cmin.as_ptr(),
                )
            };
            if res.is_null() {
                continue;
            }

            let restr = unsafe { CStr::from_ptr(res) };
            let Some(nam) = OsStr::from_bytes(restr.to_bytes())
                .to_str()
                .and_then(|path| path.strip_prefix("/dev/dsk/"))
            else {
                unsafe { free(res as *mut c_void) };
                continue;
            };

            let out = Some(
                /*
                 * If we used the "a" minor, strip the slice ("s0") from the
                 * end:
                 */
                if let Some(dev) = nam.strip_suffix("s0") { dev } else { nam }
                    .to_string(),
            );

            unsafe { free(res as *mut c_void) };
            return out;
        }

        None
    }
}

impl Drop for DevInstMinor {
    fn drop(&mut self) {
        unsafe { di_dim_fini(self.handle) }
    }
}

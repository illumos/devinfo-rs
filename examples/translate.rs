/*
 * Copyright 2024 Oxide Computer Company
 */

use anyhow::Result;

fn main() -> Result<()> {
    let dim = devinfo::DevInstMinor::new()?;

    for driver in ["blkdev", "sd"] {
        for instance in 0u32..64 {
            if let Some(nam) = dim.lookup_disk_name(driver, instance) {
                println!("{driver}{instance} -> {nam}");
            }
        }
    }

    Ok(())
}

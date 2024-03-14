/*
 * Copyright 2024 Oxide Computer Company
 */

use std::collections::HashMap;

use anyhow::Result;

struct Device {
    slot: String,
    vendor: u32,
    product: u32,
    serial: String,
}

struct Loom {
    id: String,
    devices: Vec<Device>,
}

struct UsbDevice {
    physpath: String,
    vendor: u32,
    product: u32,
    vendor_name: String,
    product_name: String,
    serialno: String,
}

impl UsbDevice {
    fn from_node(n: &devinfo::Node) -> Option<UsbDevice> {
        let mut usbvend = None;
        let mut usbprod = None;
        let mut usbprodname = None;
        let mut usbvendname = None;
        let mut usbser = None;

        let mut pw = n.props();
        while let Some(p) = pw.next().transpose().ok()? {
            match p.name().as_str() {
                "usb-vendor-id" => {
                    usbvend = Some(p);
                }
                "usb-product-id" => {
                    usbprod = Some(p);
                }
                "usb-vendor-name" => {
                    usbvendname = Some(p);
                }
                "usb-product-name" => {
                    usbprodname = Some(p);
                }
                "usb-serialno" => {
                    usbser = Some(p);
                }
                _ => {}
            }
        }

        let physpath = n.devfs_path().ok()?;
        let vendor: u32 = usbvend?.as_i32()?.try_into().unwrap();
        let product: u32 = usbprod?.as_i32()?.try_into().unwrap();
        let serialno: String = usbser?.as_cstr()?.to_str().ok()?.to_string();
        let vendor_name: String =
            usbvendname?.as_cstr()?.to_str().ok()?.to_string();
        let product_name: String =
            usbprodname?.as_cstr()?.to_str().ok()?.to_string();

        Some(UsbDevice {
            physpath,
            vendor,
            product,
            vendor_name,
            product_name,
            serialno,
        })
    }
}

fn main() -> Result<()> {
    /*
     * Mock up a set of loom definitions.
     */
    let loomdefs = vec![
        /*
         * igor:
         */
        Loom {
            id: "sn5".to_string(),
            devices: vec![
                Device {
                    slot: "probe".into(),
                    vendor: 0x483,
                    product: 0x374e,
                    serial: "003700313156501320323443".to_string(),
                },
                Device {
                    slot: "console".into(),
                    vendor: 0x403,
                    product: 0x6010,
                    serial: "FT5X1WMX".to_string(),
                },
            ],
        },
        Loom {
            id: "sn6".to_string(),
            devices: vec![
                Device {
                    slot: "probe".to_string(),
                    vendor: 0x483,
                    product: 0x3754,
                    serial: "0020000F4741500920383733".to_string(),
                },
                Device {
                    slot: "console".to_string(),
                    vendor: 0x403,
                    product: 0x6010,
                    serial: "FT5X1WXB".to_string(),
                },
            ],
        },
        Loom {
            id: "sn19".to_string(),
            devices: vec![
                Device {
                    slot: "probe".to_string(),
                    vendor: 0x483,
                    product: 0x374e,
                    serial: "000D00184741500520383733".to_string(),
                },
                Device {
                    slot: "console".to_string(),
                    vendor: 0x403,
                    product: 0x6011,
                    serial: "FT51SZA7".to_string(),
                },
            ],
        },
        Loom {
            id: "sidecar3".to_string(),
            devices: vec![Device {
                slot: "probe".to_string(),
                vendor: 0x483,
                product: 0x374e,
                serial: "0025000A4741500520383733".to_string(),
            }],
        },
        Loom {
            id: "meanwell-igor".to_string(),
            devices: vec![Device {
                slot: "probe".to_string(),
                vendor: 0x483,
                product: 0x374e,
                serial: "001D000C4741500520383733".to_string(),
            }],
        },
        /*
         * cadbury:
         */
        Loom {
            id: "sn4".to_string(),
            devices: vec![
                Device {
                    slot: "probe".into(),
                    vendor: 0x483,
                    product: 0x3754,
                    serial: "001100184741500820383733".to_string(),
                },
                Device {
                    slot: "rot".into(),
                    vendor: 0x1fc9,
                    product: 0x143,
                    serial: "XAEPYT3E3JX0D".to_string(),
                },
                Device {
                    slot: "console".into(),
                    vendor: 0x403,
                    product: 0x6010,
                    serial: "FT5WTGN6".to_string(),
                },
            ],
        },
        Loom {
            id: "sn9".to_string(),
            devices: vec![
                Device {
                    slot: "probe".to_string(),
                    vendor: 0x483,
                    product: 0x374e,
                    serial: "002F001A4741500520383733".to_string(),
                },
                Device {
                    slot: "console".to_string(),
                    vendor: 0x403,
                    product: 0x6010,
                    serial: "FT5WTIBN".to_string(),
                },
            ],
        },
        Loom {
            id: "sn14".to_string(),
            devices: vec![
                Device {
                    slot: "probe".to_string(),
                    vendor: 0x483,
                    product: 0x374e,
                    serial: "002A003D4741500520383733".to_string(),
                },
                Device {
                    slot: "console".to_string(),
                    vendor: 0x403,
                    product: 0x6011,
                    serial: "FT51SXZR".to_string(),
                },
            ],
        },
        Loom {
            id: "meanwell-cadbury".to_string(),
            devices: vec![Device {
                slot: "probe".to_string(),
                vendor: 0x483,
                product: 0x374e,
                serial: "0036001C4741500620383733".to_string(),
            }],
        },
        /*
         * lurch:
         */
        Loom {
            id: "sidecar-ls1".to_string(),
            devices: vec![Device {
                slot: "probe".to_string(),
                vendor: 0x483,
                product: 0x374e,
                serial: "0028001E4741500720383733".to_string(),
            }],
        },
        Loom {
            id: "meanwell-lurch".to_string(),
            devices: vec![Device {
                slot: "probe".to_string(),
                vendor: 0x483,
                product: 0x374e,
                serial: "0031000B4741500520383733".to_string(),
            }],
        },
        /*
         * niles:
         */
        Loom {
            id: "sidecar-niles".to_string(),
            devices: vec![Device {
                slot: "probe".to_string(),
                vendor: 0x483,
                product: 0x374f,
                serial: "002A001C4D46500F20373033".to_string(),
            }],
        },
        Loom {
            id: "psc-niles".to_string(),
            devices: vec![Device {
                slot: "probe".to_string(),
                vendor: 0x483,
                product: 0x374e,
                serial: "001C00175553500F20393256".to_string(),
            }],
        },
        Loom {
            id: "meanwell-niles".to_string(),
            devices: vec![Device {
                slot: "probe".to_string(),
                vendor: 0x483,
                product: 0x374f,
                serial: "000C001F4D46500F20373033".to_string(),
            }],
        },
    ];
    let mut looms: HashMap<String, HashMap<String, UsbDevice>> =
        Default::default();
    let mut spares: Vec<UsbDevice> = Default::default();

    let mut di = devinfo::DevInfo::new()?;

    let mut w = di.walk_node();
    'outer: while let Some(n) = w.next().transpose()? {
        if n.driver_name().as_deref() == Some("hubd") {
            continue;
        }

        let ud = if let Some(ud) = UsbDevice::from_node(&n) {
            ud
        } else {
            continue;
        };

        /*
         * Locate the correct loom for this device.
         */
        for loomdef in loomdefs.iter() {
            for devdef in loomdef.devices.iter() {
                if devdef.vendor != ud.vendor
                    || devdef.product != ud.product
                    || &devdef.serial != &ud.serialno
                {
                    continue;
                }

                /*
                 * A match!  Add this loom.
                 */
                let loom = looms.entry(loomdef.id.clone()).or_default();

                /*
                 * Add this device to the loom.
                 */
                loom.insert(devdef.slot.clone(), ud);
                continue 'outer;
            }
        }

        spares.push(ud);
    }

    for loomdef in loomdefs.iter() {
        if let Some(loom) = looms.get(&loomdef.id) {
            println!("LOOM {:?}", loomdef.id);
            for dev in loomdef.devices.iter() {
                if let Some(ud) = loom.get(&dev.slot) {
                    println!("    {}: {}", dev.slot, ud.physpath);
                    println!(
                        "    {:4x},{:4x}: {} {} ({})",
                        ud.vendor,
                        ud.product,
                        ud.vendor_name,
                        ud.product_name,
                        ud.serialno
                    );
                } else {
                    println!("    {}: MISSING", dev.slot);
                }
            }
        }
    }

    println!("SPARE DEVICES:");
    for ud in spares.iter() {
        println!("    {}", ud.physpath);
        println!(
            "    {:4x},{:4x}: {} {} ({})",
            ud.vendor, ud.product, ud.vendor_name, ud.product_name, ud.serialno
        );
    }

    Ok(())
}

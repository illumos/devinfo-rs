/*
 * Copyright 2024 Oxide Computer Company
 */

use anyhow::Result;

fn main() -> Result<()> {
    let mut di = devinfo::DevInfo::new()?;

    let mut w = di.walk_node();
    while let Some(n) = w.next().transpose()? {
        if n.driver_name().as_deref() == Some("hubd") {
            continue;
        }

        let mut pw = n.props();

        let mut usbvend = None;
        let mut usbprod = None;
        let mut usbprodname = None;
        let mut usbvendname = None;
        let mut usbser = None;
        while let Some(p) = pw.next().transpose()? {
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

        if usbvend.is_none() || usbprod.is_none() {
            continue;
        }

        println!(
            "{:4x},{:4x}: {:<20} {:<20} {}",
            usbvend.map(|p| p.as_i32().unwrap()).unwrap(),
            usbprod.map(|p| p.as_i32().unwrap()).unwrap(),
            usbvendname
                .map(|p| p.to_string().trim().to_string())
                .unwrap_or("-".into()),
            usbprodname
                .map(|p| p.to_string().trim().to_string())
                .unwrap_or("-".into()),
            usbser.map(|p| p.to_string()).unwrap_or("-".into())
        );
    }

    Ok(())
}

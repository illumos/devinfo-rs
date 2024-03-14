/*
 * Copyright 2024 Oxide Computer Company
 */

use anyhow::Result;

fn main() -> Result<()> {
    let mut di = devinfo::DevInfo::new()?;

    let mut w = di.walk_node();
    while let Some(n) = w.next().transpose()? {
        let mut ind = "".to_string();
        for _ in 1..n.depth() {
            ind.push_str("    ");
        }

        if let Some(i) = n.instance() {
            println!(
                "{}{}, instance #{} (driver name: {})",
                ind,
                n.node_name(),
                i,
                n.driver_name().unwrap()
            );
        } else {
            println!("{}{} (driver not attached)", ind, n.node_name());
        }

        if n.node_name() == "pseudo" {
            w.skip_children();
        }
    }

    Ok(())
}

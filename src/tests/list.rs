// list.rs

// Copyright (C) 2020-2021 The Nitrocli Developers
// SPDX-License-Identifier: GPL-3.0-or-later

use super::*;

#[test_device]
fn not_connected() -> anyhow::Result<()> {
  let res = Nitrocli::new().handle(&["list"])?;
  assert_eq!(res, "No Nitrokey device connected\n");

  Ok(())
}

#[test_device]
fn connected(model: nitrokey::Model) -> anyhow::Result<()> {
  let re = regex::Regex::new(
    r#"^USB path\tmodel\tserial number
([[:^space:]]+\t(Nitrokey Pro|Nitrokey Storage|Librem Key|unknown)\t0x[[:xdigit:]]+
)+$"#,
  )
  .unwrap();

  let out = Nitrocli::new().model(model).handle(&["list"])?;
  assert!(re.is_match(&out), "{}", out);
  Ok(())
}

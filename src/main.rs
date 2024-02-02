#![feature(try_find)]

use wifipass_windows::{Result, Wlan, PCWSTR};

fn main() -> Result<()> {
    let wlan = Wlan::try_new()?;
    for interface in wlan.enum_interfaces()?.as_slice().unwrap() {
        let interface_guid = interface.InterfaceGuid;
        for profile in wlan.get_profile_list(&interface_guid)?.as_slice().unwrap() {
            let name = PCWSTR::from_raw(profile.strProfileName.as_ptr());
            let root = wlan.get_profile(&interface_guid, name)?.load()?.DocumentElement()?;
            assert_eq!(root.TagName()?, "WLANProfile");

            let node = root
                .ChildNodes()?
                .First()?
                .try_find(|node| {
                    let name = node.NodeName()?;
                    Result::Ok(name == "name")
                })?
                .unwrap();
            let name = node.InnerText()?;

            let Some(node) = root.ChildNodes()?.First()?.try_find(|node| {
                let name = node.NodeName()?;
                Result::Ok(name == "MSM")
            })?
            else {
                continue;
            };

            let Some(security) = node.ChildNodes()?.First()?.try_find(|node| {
                let name = node.NodeName()?;
                Result::Ok(name == "security")
            })?
            else {
                continue;
            };

            let Some(node) = security.ChildNodes()?.First()?.try_find(|node| {
                let name = node.NodeName()?;
                Result::Ok(name == "authEncryption")
            })?
            else {
                continue;
            };

            let Some(node) = node.ChildNodes()?.First()?.try_find(|node| {
                let name = node.NodeName()?;
                Result::Ok(name == "authentication")
            })?
            else {
                continue;
            };

            let authentication = node.InnerText()?;
            if authentication == "open" {
                println!("{name} ::");
                continue;
            }

            if authentication != "WPA2" && authentication != "WPA2PSK" {
                // Skip this because we don't know how to interpret this authentication method
                continue;
            }

            let Some(node) = security.ChildNodes()?.First()?.try_find(|node| {
                let name = node.NodeName()?;
                Result::Ok(name == "sharedKey")
            })?
            else {
                continue;
            };

            let mut protected = None;
            let mut key_type = None;
            let mut key_material = None;

            for node in node.ChildNodes()?.First()? {
                let name = node.NodeName()?;
                let text = node.InnerText()?;
                if name == "keyType" {
                    key_type = Some(text);
                } else if name == "protected" {
                    protected = Some(text);
                } else if name == "keyMaterial" {
                    key_material = Some(text);
                }
            }

            let Some(((protected, key_type), key_material)) = protected.zip(key_type).zip(key_material) else {
                continue;
            };
            if protected == "false" && key_type == "passPhrase" {
                println!("{name} :: {key_material}");
            }
        }
    }
    Ok(())
}

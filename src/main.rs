fn main() -> wifipass_windows::Result<()> {
    let wlan = wifipass_windows::Wlan::try_new()?;
    for interface in wlan.enum_interfaces()?.as_slice().unwrap() {
        let interface_guid = interface.InterfaceGuid;
        for profile in wlan.get_profile_list(&interface_guid)?.as_slice().unwrap() {
            let name = wifipass_windows::PCWSTR::from_raw(profile.strProfileName.as_ptr());
            let list = wlan.get_profile(&interface_guid, name)?;
            println!("{}", list.display());
        }
    }
    Ok(())
}

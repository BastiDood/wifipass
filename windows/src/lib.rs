use windows::{
    core::{GUID, PWSTR},
    Win32::{
        Foundation::{HANDLE, INVALID_HANDLE_VALUE, WIN32_ERROR},
        NetworkManagement::WiFi,
    },
};

pub use windows::core::{Result, PCWSTR};

pub struct Wlan {
    handle: HANDLE,
}

pub struct WlanInterfaces {
    interfaces: *mut WiFi::WLAN_INTERFACE_INFO_LIST,
}

impl WlanInterfaces {
    pub fn as_slice(&self) -> Option<&[WiFi::WLAN_INTERFACE_INFO]> {
        let WiFi::WLAN_INTERFACE_INFO_LIST { InterfaceInfo, dwNumberOfItems, .. } =
            unsafe { self.interfaces.as_ref() }?;
        let count = dwNumberOfItems.clone().try_into().unwrap();
        Some(unsafe { core::slice::from_raw_parts(InterfaceInfo.as_ptr(), count) })
    }
}

impl Drop for WlanInterfaces {
    fn drop(&mut self) {
        let interfaces = core::mem::replace(&mut self.interfaces, core::ptr::null_mut()).cast();
        unsafe { WiFi::WlanFreeMemory(interfaces) };
    }
}

pub struct WlanProfileList {
    profiles: *mut WiFi::WLAN_PROFILE_INFO_LIST,
}

impl WlanProfileList {
    pub fn as_slice(&self) -> Option<&[WiFi::WLAN_PROFILE_INFO]> {
        let WiFi::WLAN_PROFILE_INFO_LIST { ProfileInfo, dwNumberOfItems, .. } = unsafe { self.profiles.as_ref() }?;
        let count = dwNumberOfItems.clone().try_into().unwrap();
        Some(unsafe { core::slice::from_raw_parts(ProfileInfo.as_ptr(), count) })
    }
}

impl Drop for WlanProfileList {
    fn drop(&mut self) {
        let profiles = core::mem::replace(&mut self.profiles, core::ptr::null_mut()).cast();
        unsafe { WiFi::WlanFreeMemory(profiles) };
    }
}

pub struct WlanProfileXml {
    xml: PWSTR,
}

impl WlanProfileXml {
    pub fn display(&self) -> impl core::fmt::Display + '_ {
        unsafe { self.xml.display() }
    }
}

impl Drop for WlanProfileXml {
    fn drop(&mut self) {
        let xml = core::mem::replace(&mut self.xml, PWSTR::null()).as_ptr().cast();
        unsafe { WiFi::WlanFreeMemory(xml) };
    }
}

impl Wlan {
    pub fn try_new() -> Result<Self> {
        let mut negotiated_version = u32::MAX;
        let mut handle = INVALID_HANDLE_VALUE;
        let result =
            unsafe { WiFi::WlanOpenHandle(WiFi::WLAN_API_VERSION, None, &mut negotiated_version, &mut handle) };
        WIN32_ERROR(result).ok()?;
        Ok(Self { handle })
    }

    pub fn enum_interfaces(&self) -> Result<WlanInterfaces> {
        let mut interfaces = core::ptr::null_mut();
        let result = unsafe { WiFi::WlanEnumInterfaces(self.handle, None, &mut interfaces) };
        WIN32_ERROR(result).ok()?;
        Ok(WlanInterfaces { interfaces })
    }

    pub fn get_profile_list(&self, interface_guid: &GUID) -> Result<WlanProfileList> {
        let mut profiles = core::ptr::null_mut();
        let result = unsafe { WiFi::WlanGetProfileList(self.handle, interface_guid, None, &mut profiles) };
        WIN32_ERROR(result).ok()?;
        Ok(WlanProfileList { profiles })
    }

    pub fn get_profile(&self, interface_guid: &GUID, profile_name: PCWSTR) -> Result<WlanProfileXml> {
        let mut xml = PWSTR::null();
        let mut flags = WiFi::WLAN_PROFILE_GET_PLAINTEXT_KEY;
        let result = unsafe {
            WiFi::WlanGetProfile(self.handle, interface_guid, profile_name, None, &mut xml, Some(&mut flags), None)
        };
        WIN32_ERROR(result).ok()?;
        Ok(WlanProfileXml { xml })
    }
}

impl Drop for Wlan {
    fn drop(&mut self) {
        let handle = core::mem::replace(&mut self.handle, INVALID_HANDLE_VALUE);
        let result = unsafe { WiFi::WlanCloseHandle(handle, None) };
        WIN32_ERROR(result).ok().unwrap();
    }
}

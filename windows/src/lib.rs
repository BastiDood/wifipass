use windows::{
    core::{GUID, HSTRING, PWSTR},
    Data::Xml::Dom::XmlDocument,
    Win32::{
        Foundation::{HANDLE, INVALID_HANDLE_VALUE, WIN32_ERROR},
        NetworkManagement::WiFi,
    },
};

pub use windows::core::{Result, PCWSTR};

pub struct Wlan {
    handle: HANDLE,
}

pub struct WlanInterfaceList {
    interfaces: &'static WiFi::WLAN_INTERFACE_INFO_LIST,
}

impl WlanInterfaceList {
    /// # Safety
    /// Takes ownership of `interfaces`.
    pub unsafe fn try_new(interfaces: *const WiFi::WLAN_INTERFACE_INFO_LIST) -> Option<Self> {
        let interfaces = interfaces.as_ref()?;
        Some(Self { interfaces })
    }

    pub fn as_slice(&self) -> Option<&[WiFi::WLAN_INTERFACE_INFO]> {
        let WiFi::WLAN_INTERFACE_INFO_LIST { ref InterfaceInfo, dwNumberOfItems, .. } = *self.interfaces;
        let count = dwNumberOfItems.try_into().unwrap();
        Some(unsafe { core::slice::from_raw_parts(InterfaceInfo.as_ptr(), count) })
    }
}

impl Drop for WlanInterfaceList {
    fn drop(&mut self) {
        let interfaces = core::ptr::from_ref(self.interfaces).cast();
        unsafe { WiFi::WlanFreeMemory(interfaces) };
    }
}

pub struct WlanProfileList {
    profiles: &'static WiFi::WLAN_PROFILE_INFO_LIST,
}

impl WlanProfileList {
    /// # Safety
    /// Takes ownership of `interfaces`.
    pub unsafe fn try_new(interfaces: *const WiFi::WLAN_PROFILE_INFO_LIST) -> Option<Self> {
        let profiles = interfaces.as_ref()?;
        Some(Self { profiles })
    }

    pub fn as_slice(&self) -> Option<&[WiFi::WLAN_PROFILE_INFO]> {
        let WiFi::WLAN_PROFILE_INFO_LIST { ref ProfileInfo, dwNumberOfItems, .. } = *self.profiles;
        let count = dwNumberOfItems.try_into().unwrap();
        Some(unsafe { core::slice::from_raw_parts(ProfileInfo.as_ptr(), count) })
    }
}

impl Drop for WlanProfileList {
    fn drop(&mut self) {
        let profiles = core::ptr::from_ref(self.profiles).cast();
        unsafe { WiFi::WlanFreeMemory(profiles) };
    }
}

pub struct WlanProfileXml {
    xml: PWSTR,
}

impl WlanProfileXml {
    pub fn load(&self) -> Result<XmlDocument> {
        let doc = XmlDocument::new()?;
        let hstring = HSTRING::from_wide(unsafe { self.xml.as_wide() })?;
        doc.LoadXml(&hstring)?;
        Ok(doc)
    }

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

    pub fn enum_interfaces(&self) -> Result<WlanInterfaceList> {
        let mut output = core::ptr::null_mut();
        let result = unsafe { WiFi::WlanEnumInterfaces(self.handle, None, &mut output) };
        WIN32_ERROR(result).ok()?;
        let interfaces = unsafe { WlanInterfaceList::try_new(output) };
        Ok(interfaces.unwrap())
    }

    pub fn get_profile_list(&self, interface_guid: &GUID) -> Result<WlanProfileList> {
        let mut output = core::ptr::null_mut();
        let result = unsafe { WiFi::WlanGetProfileList(self.handle, interface_guid, None, &mut output) };
        WIN32_ERROR(result).ok()?;
        let profiles = unsafe { WlanProfileList::try_new(output) };
        Ok(profiles.unwrap())
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

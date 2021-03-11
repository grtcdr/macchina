use crate::traits::*;
use sysctl::{Sysctl, Ctl, SysctlError};
use crate::traits::ReadoutError::{MetricNotAvailable, Other};
use objc::runtime::Object;
use objc_foundation::{INSString, NSString};
use crate::macos::mach_ffi::{vm_statistics64, IOServiceGetMatchingService, IOServiceMatching, kIOMasterPortDefault, IORegistryEntryCreateCFProperties};
use core_foundation::dictionary::{CFMutableDictionary, CFMutableDictionaryRef};
use core_foundation::base::{TCFType, ToVoid};
use crate::macos::mach_ffi::{io_registry_entry_t, IOObjectRelease};
use mach::kern_return::KERN_SUCCESS;
use std::ffi::{CString};
use core_foundation::string::{CFString};
use mach::vm_types::integer_t;

mod mach_ffi;

impl From<SysctlError> for ReadoutError {
    fn from(e: SysctlError) -> Self {
        ReadoutError::SysctlError(format!("Error while accessing system control: {:?}", e))
    }
}

pub struct MacOSBatteryReadout {
    power_info: Option<MacOSIOPMPowerSource>
}

pub struct MacOSProductReadout;

pub struct MacOSKernelReadout {
    os_type_ctl: Option<Ctl>,
    os_release_ctl: Option<Ctl>,
}

pub struct MacOSGeneralReadout {
    cpu_brand_ctl: Option<Ctl>,
    boot_time_ctl: Option<Ctl>,
    hostname_ctl: Option<Ctl>,
    hw_model_ctl: Option<Ctl>,
}

pub struct MacOSMemoryReadout {
    page_size: u64,
    physical_memory: u64,
}

#[derive(Debug, Default)]
struct MacOSIOPMPowerSource {
    battery_installed: Option<bool>,
    state_of_charge: Option<usize>,
    charging: Option<bool>,
}

pub struct MacOSPackageReadout;

impl BatteryReadout for MacOSBatteryReadout {
    fn new() -> Self {
        MacOSBatteryReadout {
            power_info: MacOSIOPMPowerSource::new().ok()
        }
    }

    fn percentage(&self) -> Result<String, ReadoutError> {
        match &self.power_info {
            Some(info) => Ok(info.state_of_charge.ok_or(MetricNotAvailable)?.to_string()),
            None => Err(MetricNotAvailable)
        }
    }

    fn status(&self) -> Result<String, ReadoutError> {
        return match &self.power_info {
            Some(info) => {
                if let Some(charging) = info.charging {
                    return match charging {
                        true => Ok(String::from("TRUE")),
                        false => Ok(String::from("FALSE"))
                    };
                }
                Err(MetricNotAvailable)
            }
            None => Err(MetricNotAvailable)
        };
    }
}

impl MacOSIOPMPowerSource {
    fn new() -> Result<Self, ReadoutError> {
        let power_source_dict = MacOSIOPMPowerSource::get_power_source_dict()?;
        let mut instance: MacOSIOPMPowerSource = std::default::Default::default();

        if let Some(battery_installed) = power_source_dict.find(&CFString::new("BatteryInstalled").to_void()) {
            let value = (*battery_installed) as *const integer_t;
            unsafe { instance.battery_installed = Some((*value) != 0); }
        } else {
            return Err(Other(String::from("No information available regarding installation status \
            of battery.")));
        }

        if let Some(state_of_charge) = power_source_dict.find(&CFString::new("StateOfCharge").to_void()) {
            let value = (*state_of_charge) as *const integer_t;
            unsafe { instance.state_of_charge = Some((*value) as usize); }
        } else {
            return Err(Other(String::from("No information available regarding state of charge.")));
        }

        if let Some(charging) = power_source_dict.find(&CFString::new("IsCharging").to_void()) {
            let value = (*charging) as *const integer_t;
            unsafe { instance.charging = Some((*value) != 0); }
        } else {
            return Err(Other(String::from("No information available regarding charging state.")));
        }

        Ok(instance)
    }

    fn get_power_source_dict() ->
                               Result<CFMutableDictionary, ReadoutError> {
        let io_service_name = CString::new("IOPMPowerSource").expect("Unable to create c string");
        let service = unsafe { IOServiceMatching(io_service_name.as_ptr()) };
        let entry: io_registry_entry_t = unsafe { IOServiceGetMatchingService(kIOMasterPortDefault, service) };
        let mut dict_data: Option<CFMutableDictionary> = None;

        if entry != 0 {
            let mut dict: CFMutableDictionaryRef = std::ptr::null_mut();
            let dict_ptr = (&mut dict) as *mut CFMutableDictionaryRef;

            let kern_return = unsafe {
                IORegistryEntryCreateCFProperties(entry, dict_ptr, std::ptr::null(), 0)
            };

            if kern_return == KERN_SUCCESS {
                dict_data = Some(unsafe { CFMutableDictionary::wrap_under_create_rule(dict) });
            }

            unsafe { IOObjectRelease(entry); }
        }

        dict_data.ok_or(ReadoutError::MetricNotAvailable)
    }
}

impl KernelReadout for MacOSKernelReadout {
    fn new() -> Self {
        MacOSKernelReadout {
            os_type_ctl: Ctl::new("kern.ostype").ok(),
            os_release_ctl: Ctl::new("kern.osrelease").ok(),
        }
    }

    fn os_release(&self) -> Result<String, ReadoutError> {
        Ok(self.os_release_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?)
    }

    fn os_type(&self) -> Result<String, ReadoutError> {
        Ok(self.os_type_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?)
    }
}

impl GeneralReadout for MacOSGeneralReadout {
    fn new() -> Self {
        MacOSGeneralReadout {
            cpu_brand_ctl: Ctl::new("machdep.cpu.brand_string").ok(),
            boot_time_ctl: Ctl::new("kern.boottime").ok(),
            hostname_ctl: Ctl::new("kern.hostname").ok(),
            hw_model_ctl: Ctl::new("hw.model").ok(),
        }
    }

    fn username(&self) -> Result<String, ReadoutError> {
        crate::shared::whoami()
    }

    fn hostname(&self) -> Result<String, ReadoutError> {
        Ok(self.hostname_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?)
    }

    fn desktop_environment(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Aqua"))
    }

    fn window_manager(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Quartz Compositor"))
    }

    fn terminal(&self) -> Result<String, ReadoutError> {
        if let Some(terminal_env) = std::env::var("TERM").ok() {
            return Ok(terminal_env);
        }

        crate::shared::terminal()
    }

    fn shell(&self, shorthand: bool) -> Result<String, ReadoutError> {
        crate::shared::shell(shorthand)
    }

    fn cpu_model_name(&self) -> Result<String, ReadoutError> {
        Ok(self.cpu_brand_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?)
    }

    fn uptime(&self) -> Result<String, ReadoutError> {
        use std::time::{Duration, SystemTime, UNIX_EPOCH};
        use libc::timeval;

        let time = self.boot_time_ctl.as_ref().ok_or(MetricNotAvailable)?.value_as::<timeval>()?;
        let duration = Duration::new(time.tv_sec as u64, (time.tv_usec * 1000) as
            u32);
        let bootup_timestamp = UNIX_EPOCH + duration;

        if let Ok(duration) = SystemTime::now().duration_since(bootup_timestamp) {
            let seconds_since_boot = duration.as_secs_f64();
            return Ok(seconds_since_boot.to_string());
        }

        Err(ReadoutError::Other(String::from("Error calculating boot time since unix \
            epoch.")))
    }

    fn machine(&self) -> Result<String, ReadoutError> {
        let product_readout = MacOSProductReadout;

        let version = product_readout.version()?;
        let name = product_readout.product()?;
        let major_version_name = unsafe { macos_version_to_name() };
        let mac_model = self.hw_model_ctl.as_ref().ok_or(MetricNotAvailable)?.value_string()?;

        Ok(format!("{} ({} {} {})", mac_model, name, version, major_version_name))
    }
}

impl MemoryReadout for MacOSMemoryReadout {
    fn new() -> Self {
        let page_size = match Ctl::new("hw.pagesize").unwrap().value().unwrap() {
            sysctl::CtlValue::S64(s) => s,
            _ => panic!("Could not get vm page size.")
        };

        let physical_mem = match Ctl::new("hw.memsize").unwrap().value().unwrap() {
            sysctl::CtlValue::S64(s) => s,
            _ => panic!("Could not get physical memory size.")
        };

        MacOSMemoryReadout {
            page_size,
            physical_memory: physical_mem,
        }
    }

    fn total(&self) -> Result<u64, ReadoutError> {
        Ok(self.physical_memory / 1024)
    }

    fn free(&self) -> Result<u64, ReadoutError> {
        let vm_stats = MacOSMemoryReadout::mach_vm_stats()?;
        let free_count: u64 = (vm_stats.free_count + vm_stats.inactive_count - vm_stats.speculative_count) as u64;

        Ok(((free_count * self.page_size) / 1024) as u64)
    }

    fn reclaimable(&self) -> Result<u64, ReadoutError> {
        let vm_stats = MacOSMemoryReadout::mach_vm_stats()?;
        Ok((vm_stats.purgeable_count as u64 * self.page_size / 1024) as u64)
    }

    fn used(&self) -> Result<u64, ReadoutError> {
        let vm_stats = MacOSMemoryReadout::mach_vm_stats()?;
        let used: u64 = ((vm_stats.active_count + vm_stats.wire_count) as u64 * self.page_size /
            1024) as u64;

        Ok(used)
    }
}

impl MacOSMemoryReadout {
    fn mach_vm_stats() -> Result<vm_statistics64, ReadoutError> {
        use mach::message::{mach_msg_type_number_t};
        use mach::kern_return::KERN_SUCCESS;
        use mach::vm_types::{integer_t};
        use mach_ffi::*;

        const HOST_VM_INFO_COUNT: mach_msg_type_number_t =
            (std::mem::size_of::<vm_statistics64>() /
                std::mem::size_of::<integer_t>()) as u32;

        const HOST_VM_INFO64: integer_t = 4;

        let mut vm_stat: vm_statistics64 = std::default::Default::default();
        let vm_stat_ptr: *mut vm_statistics64 = &mut vm_stat;
        let mut count: mach_msg_type_number_t = HOST_VM_INFO_COUNT;

        let ret_val = unsafe {
            host_statistics64(mach_host_self(), HOST_VM_INFO64, vm_stat_ptr as *mut integer_t,
                              &mut
                                  count as *mut mach_msg_type_number_t)
        };

        if ret_val == KERN_SUCCESS {
            return Ok(vm_stat);
        }

        Err(ReadoutError::Other(String::from("Could not retrieve vm statistics from host.")))
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct NSOperatingSystemVersion {
    major_version: u64,
    minor_version: u64,
    patch_version: u64,
}

impl Into<String> for NSOperatingSystemVersion {
    fn into(self) -> String {
        format!("{}.{}.{}", self.major_version, self.minor_version, self.patch_version)
    }
}

impl ProductReadout for MacOSProductReadout {
    fn new() -> Self {
        MacOSProductReadout
    }

    fn version(&self) -> Result<String, ReadoutError> {
        Ok(MacOSProductReadout::operating_system_version().into())
    }

    fn vendor(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Apple"))
    }

    fn family(&self) -> Result<String, ReadoutError> {
        Ok(String::from("Unix, Macintosh"))
    }

    fn name(&self) -> Result<String, ReadoutError> {
        let process_class = class![NSProcessInfo];
        let process_info: *mut Object = unsafe { msg_send![process_class, processInfo] };
        let version_string: *const NSString = unsafe {
            msg_send![process_info, operatingSystemVersionString]
        };

        let version_string = unsafe { (*version_string).as_str() };

        Ok(String::from(version_string))
    }

    fn product(&self) -> Result<String, ReadoutError> {
        Ok(String::from("macOS"))
    }
}

impl MacOSProductReadout {
    fn operating_system_version() -> NSOperatingSystemVersion {
        let class = class![NSProcessInfo];
        let process_info: *mut Object = unsafe { msg_send![class, processInfo] };
        unsafe { msg_send![process_info, operatingSystemVersion] }
    }
}

impl PackageReadout for MacOSPackageReadout {
    fn new() -> Self {
        MacOSPackageReadout
    }

    /// This methods check the `/usr/local/Cellar` and `/usr/local/Caskroom` folders which will
    /// contain all installed packages when using the Homebrew package manager. A manually call via
    /// `homebrew list` would be too expensive, since it is pretty slow.
    fn count_pkgs(&self) -> Result<String, ReadoutError> {
        use std::fs::read_dir;
        use std::path::Path;

        let homebrew_root = Path::new("/usr/local");
        let cellar_folder = homebrew_root.join("Cellar");
        let caskroom_folder = homebrew_root.join("Caskroom");

        let cellar_count = match read_dir(cellar_folder) {
            Ok(read_dir) => read_dir.count(),
            Err(_) => 0
        };

        let caskroom_count = match read_dir(caskroom_folder) {
            Ok(read_dir) => read_dir.count(),
            Err(_) => 0
        };

        let total = cellar_count + caskroom_count;
        if total == 0 { return Err(MetricNotAvailable); }

        Ok(format!("{}", total))
    }
}

unsafe fn macos_version_to_name() -> &'static str {
    let version = MacOSProductReadout::operating_system_version();

    match (version.major_version, version.minor_version) {
        (10, 1) => "Puma",
        (10, 2) => "Jaguar",
        (10, 3) => "Panther",
        (10, 4) => "Tiger",
        (10, 5) => "Leopard",
        (10, 6) => "Snow Leopard",
        (10, 7) => "Lion",
        (10, 8) => "Mountain Lion",
        (10, 9) => "Mavericks",
        (10, 10) => "Yosemite",
        (10, 11) => "El Capitan",
        (10, 12) => "Sierra",
        (10, 13) => "High Sierra",
        (10, 14) => "Mojave",
        (10, 15) => "Catalina",
        (11, _) => "Big Sur",
        _ => "Unknown"
    }
}
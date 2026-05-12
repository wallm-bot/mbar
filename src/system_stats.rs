#[cxx_qt::bridge]
mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(f64, cpu_usage)]
        #[qproperty(f64, ram_usage)]
        #[qproperty(f64, disk_usage)]
        #[qproperty(f64, gpu_usage)]
        // Detail values (absolute)
        #[qproperty(f64, ram_gb)]
        #[qproperty(f64, disk_gb)]
        #[qproperty(f64, gpu_vram_gb)]
        type SystemStatsBackend = super::SystemStatsBackendRust;

        #[qinvokable]
        fn update_stats(self: Pin<&mut SystemStatsBackend>);
    }
}

pub struct SystemStatsBackendRust {
    cpu_usage: f64,
    ram_usage: f64,
    disk_usage: f64,
    gpu_usage: f64,
    ram_gb: f64,
    disk_gb: f64,
    gpu_vram_gb: f64,
    sys: sysinfo::System,
    disks: sysinfo::Disks,
    nvml: Option<nvml_wrapper::Nvml>,
}

impl Default for SystemStatsBackendRust {
    fn default() -> Self {
        let mut sys = sysinfo::System::new_with_specifics(
            sysinfo::RefreshKind::nothing()
                .with_cpu(sysinfo::CpuRefreshKind::nothing().with_cpu_usage())
                .with_memory(sysinfo::MemoryRefreshKind::everything()),
        );
        sys.refresh_cpu_usage();
        sys.refresh_memory();
        let disks = sysinfo::Disks::new_with_refreshed_list();
        let nvml = nvml_wrapper::Nvml::init().ok();
        Self {
            cpu_usage: 0.0,
            ram_usage: 0.0,
            disk_usage: 0.0,
            gpu_usage: 0.0,
            ram_gb: 0.0,
            disk_gb: 0.0,
            gpu_vram_gb: 0.0,
            sys,
            disks,
            nvml,
        }
    }
}

use cxx_qt::CxxQtType;

impl qobject::SystemStatsBackend {
    pub fn update_stats(mut self: std::pin::Pin<&mut Self>) {
        // --- CPU ---
        self.as_mut().rust_mut().sys.refresh_cpu_usage();
        let cpu = self.as_mut().rust_mut().sys.global_cpu_usage() as f64;

        // --- RAM ---
        self.as_mut().rust_mut().sys.refresh_memory();
        let total_mem = self.as_mut().rust_mut().sys.total_memory();
        let used_mem = self.as_mut().rust_mut().sys.used_memory();
        let ram = if total_mem > 0 {
            used_mem as f64 / total_mem as f64 * 100.0
        } else {
            0.0
        };
        let ram_gb = used_mem as f64 / 1_073_741_824.0;

        // --- Disk ---
        self.as_mut().rust_mut().disks.refresh(true);
        let (total_disk, used_disk) = self.as_mut().rust_mut().disks.iter().fold((0u64, 0u64), |(t, u), d| {
            (
                t + d.total_space(),
                u + d.total_space() - d.available_space(),
            )
        });
        let disk = if total_disk > 0 {
            used_disk as f64 / total_disk as f64 * 100.0
        } else {
            0.0
        };
        let disk_gb = used_disk as f64 / 1_073_741_824.0;

        // --- GPU (NVIDIA NVML) ---
        let (gpu, gpu_vram_gb) = if let Some(ref nvml) = self.as_mut().rust_mut().nvml {
            match nvml.device_by_index(0u32) {
                Ok(device) => {
                    let util = device.utilization_rates()
                        .map(|u| u.gpu as f64)
                        .unwrap_or(0.0);
                    let vram = device.memory_info()
                        .map(|m| m.used as f64 / 1_073_741_824.0)
                        .unwrap_or(0.0);
                    (util, vram)
                }
                Err(_) => (0.0, 0.0),
            }
        } else {
            (0.0, 0.0)
        };

        self.as_mut().set_cpu_usage(cpu);
        self.as_mut().set_ram_usage(ram);
        self.as_mut().set_disk_usage(disk);
        self.as_mut().set_gpu_usage(gpu);
        self.as_mut().set_ram_gb(ram_gb);
        self.as_mut().set_disk_gb(disk_gb);
        self.as_mut().set_gpu_vram_gb(gpu_vram_gb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_initialization() {
        let stats = SystemStatsBackendRust::default();
        assert_eq!(stats.cpu_usage, 0.0);
        assert_eq!(stats.ram_usage, 0.0);
        assert_eq!(stats.disk_usage, 0.0);
        assert_eq!(stats.gpu_usage, 0.0);
        assert_eq!(stats.ram_gb, 0.0);
        assert_eq!(stats.disk_gb, 0.0);
        assert_eq!(stats.gpu_vram_gb, 0.0);
    }
}

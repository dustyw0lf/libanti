use std::fs;

use raw_cpuid::CpuId;

use crate::error::{Error, Result};

pub fn is_virt_cpu() -> Result<bool> {
    let cpuid = CpuId::new();
    let brand = cpuid
        .get_processor_brand_string()
        .ok_or_else(|| Error::CpuId("failed to get processor brand string".to_string()))?;
    let brand = brand.as_str();

    if brand.contains("Intel") || brand.contains("AMD") {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn is_virt_disk() -> Result<bool> {
    let paths = fs::read_dir("/dev/disk/by-path")?;

    for path in paths {
        let path = path?.path();
        let path = path.to_string_lossy();
        if path.contains("vmbus") || path.contains("scsi") {
            return Ok(true);
        }
    }
    Ok(false)
}

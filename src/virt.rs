use raw_cpuid::CpuId;

pub fn is_virt_cpu() -> Result<bool, Box<dyn std::error::Error>> {
    let cpuid = CpuId::new();
    let brand = cpuid
        .get_processor_brand_string()
        .ok_or_else(|| "Failed to get processor brand ID")?;
    let brand = brand.as_str();

    if brand.contains("Intel") || brand.contains("AMD") {
        Ok(false)
    } else {
        Ok(true)
    }
}

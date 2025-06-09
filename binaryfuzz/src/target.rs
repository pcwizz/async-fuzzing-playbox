use gimli::{AttributeValue, Dwarf, DwarfSections, EndianSlice, LittleEndian, SectionId};
use object::{Object, ObjectSection};

#[derive(Debug)]
struct AsyncFunctionEntry {
    name: String,
    address: u64,
    mangled_name: String,
    is_wrapper: bool,
    is_generator: bool,
}

fn find_specific_async_function(
    dwarf: &Dwarf<EndianSlice<'_, LittleEndian>>,
    target_name: &str,
) -> Result<Vec<AsyncFunctionEntry>, gimli::Error> {
    let mut results = Vec::new();
    let mut units = dwarf.units();

    while let Some(header) = units.next()? {
        let unit = dwarf.unit(header)?;
        let mut entries = unit.entries();

        while let Some((_, entry)) = entries.next_dfs()? {
            if entry.tag() == gimli::DW_TAG_subprogram {
                if let Some(func_info) = analyze_function_entry(dwarf, &unit, entry, target_name)? {
                    results.push(func_info);
                }
            }
        }
    }

    Ok(results)
}

fn analyze_function_entry(
    dwarf: &Dwarf<EndianSlice<LittleEndian>>,
    unit: &gimli::Unit<EndianSlice<LittleEndian>>,
    entry: &gimli::DebuggingInformationEntry<EndianSlice<LittleEndian>>,
    target_name: &str,
) -> Result<Option<AsyncFunctionEntry>, gimli::Error> {
    // Get both mangled and demangled names
    let mangled_name = get_linkage_name(dwarf, unit, entry)?;
    let demangled_name = get_name(dwarf, unit, entry)?;
    let address = get_low_pc(dwarf, unit, entry)?;

    if let (Some(address), name) = (address, demangled_name.or(mangled_name.clone())) {
        if let Some(name) = name {
            // Look for the specific function name in various forms
            if is_target_async_function(&name, &mangled_name, target_name) {
                return Ok(Some(AsyncFunctionEntry {
                    name: name.clone(),
                    address,
                    mangled_name: mangled_name.unwrap_or_default(),
                    is_wrapper: is_async_wrapper(&name),
                    is_generator: is_generator_function(&name),
                }));
            }
        }
    }

    Ok(None)
}

fn is_target_async_function(demangled: &str, mangled: &Option<String>, target: &str) -> bool {
    // Direct match
    if demangled.contains(&format!("{}::", target)) || demangled.contains(&format!("::{}", target))
    {
        // Ensure it's async-related
        return demangled.contains("async")
            || demangled.contains("Future")
            || demangled.contains("Generator")
            || demangled.contains("{{closure}}")
            || (mangled.as_ref().map_or(false, |m| m.contains("async")));
    }

    // Check mangled name patterns
    if let Some(mangled_name) = mangled {
        // Rust mangled names often contain the function name
        if mangled_name.contains(target)
            && (mangled_name.contains("async") || mangled_name.contains("Future"))
        {
            return true;
        }
    }

    false
}

fn is_async_wrapper(name: &str) -> bool {
    // The outer wrapper function that returns the Future
    !name.contains("{{closure}}")
        && !name.contains("Generator")
        && !name.contains("resume")
        && (name.contains("async") || name.contains("Future"))
}

fn is_generator_function(name: &str) -> bool {
    // The actual async state machine implementation
    name.contains("Generator")
        || name.contains("{{closure}}")
        || name.contains("resume")
        || name.contains("from_generator")
}

fn get_name(
    dwarf: &Dwarf<EndianSlice<LittleEndian>>,
    unit: &gimli::Unit<EndianSlice<LittleEndian>>,
    entry: &gimli::DebuggingInformationEntry<EndianSlice<LittleEndian>>,
) -> Result<Option<String>, gimli::Error> {
    if let Some(attr) = entry.attr(gimli::DW_AT_name)? {
        if let AttributeValue::DebugStrRef(offset) = attr.value() {
            let name = dwarf.debug_str.get_str(offset)?;
            return Ok(Some(name.to_string_lossy().into_owned()));
        }
    }
    Ok(None)
}

fn get_linkage_name(
    dwarf: &Dwarf<EndianSlice<LittleEndian>>,
    unit: &gimli::Unit<EndianSlice<LittleEndian>>,
    entry: &gimli::DebuggingInformationEntry<EndianSlice<LittleEndian>>,
) -> Result<Option<String>, gimli::Error> {
    if let Some(attr) = entry.attr(gimli::DW_AT_linkage_name)? {
        if let AttributeValue::DebugStrRef(offset) = attr.value() {
            let name = dwarf.debug_str.get_str(offset)?;
            return Ok(Some(name.to_string_lossy().into_owned()));
        }
    }
    Ok(None)
}

fn get_low_pc(
    dwarf: &Dwarf<EndianSlice<LittleEndian>>,
    unit: &gimli::Unit<EndianSlice<LittleEndian>>,
    entry: &gimli::DebuggingInformationEntry<EndianSlice<LittleEndian>>,
) -> Result<Option<u64>, gimli::Error> {
    if let Some(attr) = entry.attr(gimli::DW_AT_low_pc)? {
        if let AttributeValue::Addr(addr) = attr.value() {
            return Ok(Some(addr));
        }
    }
    Ok(None)
}

fn find_async_foo_boundaries(
    binary_data: &[u8],
) -> Result<AsyncFunctionBoundaries, Box<dyn std::error::Error>> {
    let object = object::File::parse(binary_data)?;

    let load_section = |section: SectionId| -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        object
            .section_by_name(section.name())
            .and_then(|s| s.uncompressed_data().ok())
            .map(|cow| cow.into_owned())
            .ok_or_else(|| format!("Section {} not found", section.name()).into())
    };

    let dwarf_sections = DwarfSections::load(load_section)?;

    let dwarf = dwarf_sections.borrow(|section| EndianSlice::new(&section, LittleEndian));

    let entries = find_specific_async_function(&dwarf, "foo")?;

    let mut boundaries = AsyncFunctionBoundaries::default();

    for entry in entries {
        if entry.is_wrapper {
            // This is the function entry point - where you want to snapshot
            boundaries.entry_point = Some(entry.address);
        } else if entry.is_generator {
            // This is the actual async execution
            boundaries.generator_start = Some(entry.address);
        }

        println!(
            "Found: {} at 0x{:x} (wrapper: {}, generator: {})",
            entry.name, entry.address, entry.is_wrapper, entry.is_generator
        );
    }

    Ok(boundaries)
}

#[derive(Default, Debug)]
struct AsyncFunctionBoundaries {
    entry_point: Option<u64>,     // Where the async fn is called
    generator_start: Option<u64>, // Where the async body starts executing
}

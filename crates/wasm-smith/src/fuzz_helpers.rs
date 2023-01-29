use arbitrary::Unstructured;
use crate::InstructionKinds;

///export arbitrary creation for generator
pub fn export_swarmconfig_arbitrary(u: &mut Unstructured)  -> crate::config::SwarmConfig {
        const MAX_MAXIMUM: usize = 1000;

        let reference_types_enabled: bool = u.arbitrary().unwrap();
        let max_tables = if reference_types_enabled { 100 } else { 1 };

        crate::config::SwarmConfig {
            max_types: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_imports: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_tags: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_funcs: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_globals: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_exports: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_element_segments: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_elements: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_data_segments: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_instructions: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_memories: u.int_in_range(0..=100).unwrap(),
            max_tables,
            max_memory_pages: u.arbitrary().unwrap(),
            min_uleb_size: u.int_in_range(0..=5).unwrap(),
            bulk_memory_enabled: reference_types_enabled || u.arbitrary().unwrap(),
            reference_types_enabled,
            tail_call_enabled: u.arbitrary().unwrap(),
            simd_enabled: u.arbitrary().unwrap(),
            multi_value_enabled: u.arbitrary().unwrap(),
            max_aliases: u.int_in_range(0..=MAX_MAXIMUM).unwrap(),
            max_nesting_depth: u.int_in_range(0..=10).unwrap(),
            saturating_float_to_int_enabled: u.arbitrary().unwrap(),
            sign_extension_enabled: u.arbitrary().unwrap(),
            allowed_instructions: {
                use flagset::Flags;
                let mut allowed = Vec::new();
                for kind in crate::core::InstructionKind::LIST {
                    if u.arbitrary().unwrap() {
                        allowed.push(*kind);
                    }
                }
                InstructionKinds::new(&allowed)
            },
            table_max_size_required: u.arbitrary().unwrap(),
            max_table_elements: u.int_in_range(0..=1_000_000).unwrap(),

            // These fields, unlike the ones above, are less useful to set.
            // They either make weird inputs or are for features not widely
            // implemented yet so they're turned off by default.
            min_types: 0,
            min_imports: 0,
            min_tags: 0,
            min_funcs: 0,
            min_globals: 0,
            min_exports: 0,
            min_element_segments: 0,
            min_elements: 0,
            min_data_segments: 0,
            min_memories: 0,
            min_tables: 0,
            memory_max_size_required: false,
            max_instances: 0,
            max_modules: 0,
            max_components: 0,
            max_values: 0,
            memory_offset_choices: (75, 24, 1),
            allow_start_export: true,
            relaxed_simd_enabled: false,
            exceptions_enabled: false,
            memory64_enabled: false,
            max_type_size: 1000,
            canonicalize_nans: false,
            available_imports: None,
            threads_enabled: false,
            export_everything: false,
            disallow_traps: false,
        }

}

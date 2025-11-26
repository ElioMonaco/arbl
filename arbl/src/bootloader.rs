pub fn list_boot_options(system_table: &SystemTable<Boot>) {
    // Code to list available boot options
    system_table.stdout().write_str("Listing boot options...\n").unwrap();
}
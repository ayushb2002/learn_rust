mod array_and_strings;
mod enums_data;
mod options;
mod pub_functions;
mod results;
mod stmt_and_loops;
mod structures;
mod templates;
fn main() {
    pub_functions::call_is_even();
    stmt_and_loops::if_else_statement();
    array_and_strings::arrays_and_slices();
    structures::structures();
    enums_data::compare_enum_values();
    templates::vectors();
    templates::maps();
    options::options();
    results::results()
}

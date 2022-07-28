use std::{fs, io::Write};

use carbon_lang_compiler::{
    lexer::tokenize::tokenize,
    package_generator::command_builder::function_block::build_function_command,
    parser::{decorator::decorate_token, pipeline::build_whole_file},
    shared::package_generation::{
        package_descriptor::PackageMetadata, relocation_reference::RelocatableCommandList,
    },
};
use chrono::Local;
use console::style;

use crate::{models::command_args::CompileCommandArgs, STDOUT};

pub fn compile_package(args: CompileCommandArgs) {
    // Calculate procedure time
    let time_start = Local::now();

    // Find out whether we are going to compile a directory or files
    if args.input_path.is_dir() {
        // Fill code later
    } else {
        let file_content = fs::read_to_string(&args.input_path);
        if file_content.is_ok() {
            let tokens = tokenize(file_content.unwrap().as_str(), true);
            let decorated_tokens = decorate_token(tokens);
            let tree_result = build_whole_file(decorated_tokens, args.entry_function);

            let metadata = PackageMetadata {
                variable_slot_alignment: 2,
                data_alignment: 8,
                command_alignment: 4,
                domain_layer_count_alignment: 2,
                address_alignment: 8,
                entry_point_offset: 5,
            };

            if tree_result.is_ok() {
                let tree = tree_result.unwrap();

                let mut output = RelocatableCommandList::new();
                for func in &tree.functions {
                    output.combine(build_function_command(func, &metadata));
                }

                output.calculate_ref_to_target(metadata.serialize().len());
                output.apply_relocation(metadata.address_alignment);

                let mut output_file = fs::File::create(&args.output_path).unwrap();
                output_file
                    .write_all(metadata.serialize().as_slice())
                    .unwrap();
                output_file
                    .write_all(&output.commands.as_slice())
                    .unwrap();

                let time_spanned = Local::now() - time_start;
                STDOUT.write_line(format!("{}: Compilation completed in {}s", style("Success").green(), ((time_spanned.num_milliseconds() as f64 / 1000 as f64) as f64)).as_str()).unwrap();
            } else {
                STDOUT.write_line(format!("{}: Errors occurred during package compilation", style("Error").red()).as_str()).unwrap();
            }
        } else {
            STDOUT
                .write_line(
                    format!(
                        "{}: couldn't open file \"{}\"",
                        style("Error").red(),
                        style(args.input_path.as_path().to_str().unwrap()).green()
                    )
                    .as_str(),
                )
                .unwrap();
        }
    }
}

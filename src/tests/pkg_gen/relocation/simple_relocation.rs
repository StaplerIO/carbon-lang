// use std::io::Write;

use crate::lexer::tokenize::tokenize;
use crate::package_generator::command_builder::action_block::action_block_builder;
use crate::parser::decorator::decorate_token;
use crate::shared::ast::action::ActionBlock;
use crate::shared::package_generation::package_descriptor::PackageMetadata;

#[test]
fn no_function_relocation() {
    let tokens = tokenize(
        "decl var number foo;\
                   decl var number bar;\
                   foo = 0;\
                   bar = 2 + 4;\
                   while (foo < bar) {\
                       foo = foo + 1;\
                   }
                   foo = 4;
                   if (foo > 1202) {
                        foo = foo + 1;
                   } else {
                        foo = foo + 2;
                   }",
        true);

    let actions = crate::parser::builder::blocks::action_block::action_block_builder(
        decorate_token(tokens),
    );

    let metadata = PackageMetadata {
        variable_slot_alignment: 2,
        data_alignment: 4,
        command_alignment: 2,
        entry_point_offset: 5,
        domain_layer_count_alignment: 2,
        address_alignment: 4,
    };

    let mut target = action_block_builder(&ActionBlock { actions }, false, &vec![], &metadata);

    // Write file
    let mut file = std::fs::File::create("F:\\test.cbp").unwrap();

    let mut bytes = metadata.serialize();

    target.calculate_ref_to_target(metadata.serialize().len());
    target.apply_relocation(metadata.address_alignment);

    // bytes.extend(target.commands.clone());
    // file.write_all(bytes.as_slice()).unwrap();

    // println!("{}", itertools::Itertools::join(&mut target.commands.iter(), ", "));
}

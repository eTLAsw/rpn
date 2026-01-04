pub struct Cmd {
    pub command: &'static str,
    pub help: &'static str,
}

pub struct Category {
    pub category: &'static str,
    pub commands: &'static [Cmd],
}

const ALL_CATEGORIES: &[&Category] = &[
    &crate::basic_math::HELP,
    &crate::logic_operators::HELP,
    &crate::modes::HELP,
    &crate::stack_manipulations::HELP,
];

pub fn print_help() {
    for category in ALL_CATEGORIES {
        println!("Category: {}", category.category);
        for cmd in category.commands {
            println!("  {}: {}", cmd.command, cmd.help);
        }
        println!();
    }
}

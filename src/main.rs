use cock_lib::bin_modules::standard_prompt;

fn main() {
    let handler = standard_prompt::cock_handler_build();

    println!("{}", handler)
}

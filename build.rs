use globmatch;

fn main() {
    // Find all capnp files under src and add them to the capnp command builder
    let capnp_file_builder = globmatch::Builder::new("**/*.capnp")
        .build("src");

    match capnp_file_builder {
        Ok(c) => {
            let paths: Vec<_> = c
                .into_iter()
                .filter_entry(|p| !p.ends_with("rust.capnp"))
                .flatten()
                .collect();

            if paths.len() > 0 {
                let mut capnp_command = capnpc::CompilerCommand::new();
                let mut capnp_command_builder = capnp_command.src_prefix("src").output_path("src").import_path("src");

                for path in paths {
                    println!("{}", path.to_str().unwrap());
                    capnp_command_builder = capnp_command_builder.file(path);
                }

                capnp_command_builder.run().expect("schema compiler command");
            }
        }
        Err(_) => {}
    }
}
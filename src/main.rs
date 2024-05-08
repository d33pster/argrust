use argrust;
use rustypath::RPath;
fn main() {
    let mut args = argrust::Arguments::new(std::env::args().skip(1).collect());
    
    args.add("--test", argrust::ArgumentDescription::new().short("-t"));
    // args.add("--test2", argrust::ArgumentDescription::new().short("-t2"));

    args.analyse();

    if args.ifarg("--test") {
        let output_install_gcl = std::process::Command::new("cargo")
            .arg("install")
            .arg("gcl")
            .output()
            .expect("Failed to install gcl.");

        if output_install_gcl.status.success() {
            let mut check = false;
            if let Ok(m) = std::fs::metadata(RPath::pwd().join("argrust").convert_to_string()) {
                if m.is_dir() {
                    check = true;
                }
            }
            if !check {
                let output_clone_argrust = std::process::Command::new("gcl")
                    .arg("d33pster")
                    .arg("argrust")
                    .output()
                    .expect("Failed to clone argrust.");

                if output_clone_argrust.status.success() {
                    // go to the new dir
                    if let Err(err) = std::env::set_current_dir(RPath::pwd().join("argrust").convert_to_string()) {
                        eprintln!("Err: {}", err);
                        std::process::exit(1);
                    }

                    let output_test = std::process::Command::new("cargo")
                        .arg("test")
                        .output()
                        .expect("Failed to run tests");

                    if output_test.status.success() {
                        println!("{}", String::from_utf8_lossy(&output_test.stdout));
                        std::process::exit(0);
                    } else {
                        eprintln!("{}", String::from_utf8_lossy(&output_test.stderr));
                        std::process::exit(1);
                    }
                } else {
                    eprintln!("{}", String::from_utf8_lossy(&output_clone_argrust.stderr));
                    std::process::exit(1);
                }
            } else {
                // go to the new dir
                if let Err(err) = std::env::set_current_dir(RPath::pwd().join("argrust").convert_to_string()) {
                    eprintln!("Err: {}", err);
                    std::process::exit(1);
                }

                let output_test = std::process::Command::new("cargo")
                    .arg("test")
                    .output()
                    .expect("Failed to run tests");

                if output_test.status.success() {
                    println!("{}", String::from_utf8_lossy(&output_test.stdout));
                    std::process::exit(0);
                } else {
                    eprintln!("{}", String::from_utf8_lossy(&output_test.stderr));
                    std::process::exit(1);
                }
            }
        } else {
            eprint!("{}", String::from_utf8_lossy(&output_install_gcl.stderr));
            std::process::exit(1);
        }
    }
}

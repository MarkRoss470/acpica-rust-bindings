use std::{
    env,
    ffi::{OsStr, OsString},
    fs::{self, DirEntry},
    path::{PathBuf, Path},
    process::Command,
};

fn main() {
    #[cfg(not(unix))]
    {
        println!("cargo:warning=Building acpica_bindings on non-unix platforms is not supported. Build may not succeed.");
    }

    let acpica_dir = find_source_dir();
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir_path = PathBuf::from(out_dir.clone());

    update_source(acpica_dir.clone());

    let object_files = compile(&acpica_dir, &out_dir_path);
    create_archive(out_dir_path, object_files);
}

fn find_source_dir() -> PathBuf {
    let dirs = fs::read_dir(".").unwrap();

    for dir in dirs {
        let dir = dir.unwrap();

        // Don't use a file as the source directory
        if !dir.file_type().unwrap().is_dir() {
            continue;
        }

        if dir
            .file_name()
            .into_string()
            .unwrap_or(String::new())
            .starts_with("acpica-")
        {
            return dir.path();
        }
    }

    panic!("Couldn't find ACPICA source directory. The directory should be in the crate root and start with 'acpica-'.");
}

/// The ACPICA sources link to the system libc by default.
/// This is an issue for rust OS projects, which may not link to libc at all.
/// Thankfully, by removing definitions of the macros `ACPI_USE_SYSTEM_CLIBRARY` and `ACPI_USE_STANDARD_HEADERS`,
/// ACPICA will link to its own libc functions instead.
///
/// This function comments out all such definitions.
fn update_source(acpica_dir: PathBuf) {
    let mut platforms_path = acpica_dir.clone();
    platforms_path.push("source/include/platform");

    let platforms = fs::read_dir(platforms_path)
        .expect("Source directory should contain sub-directory 'source/include/platform'");

    for platform_file in platforms {
        let platform_file = platform_file.unwrap();
        let contents = fs::read_to_string(platform_file.path()).unwrap();

        let contents = contents.replace(
            "\n#define ACPI_USE_SYSTEM_CLIBRARY",
            "\n// #define ACPI_USE_SYSTEM_CLIBRARY",
        );
        let contents = contents.replace(
            "\n#define ACPI_USE_STANDARD_HEADERS",
            "\n// #define ACPI_USE_STANDARD_HEADERS",
        );

        fs::write(platform_file.path(), contents).unwrap();
    }
}

/// Creates a static library containing the given object files, writing it to `out_dir/libacpica.a`
fn create_archive(out_dir: PathBuf, object_files: Vec<PathBuf>) {
    let mut archive_path = out_dir.clone();
    archive_path.push("libacpica.a");

    let mut ar_command = Command::new("ar");
    ar_command.arg("-rcs").arg(archive_path);
    for obj in object_files {
        ar_command.arg(obj);
    }

    assert!(ar_command.status().unwrap().success());

    println!("cargo:rustc-link-lib=static=acpica");

    println!(
        "cargo:rustc-link-search=native={}",
        env::var("OUT_DIR").unwrap()
    );
}

/// Runs GCC on all the C files making up the ACPICA kernel subsystem,
/// returning the filepaths of the generated object files.
fn compile(acpica_dir: &Path, out_dir: &Path) -> Vec<PathBuf> {
    let mut object_files = vec![];

    let mut components_path = acpica_dir.to_path_buf();
    components_path.push("source/components");

    // Only the 'components' directory is needed for the kernel subsystem - the other directories are for cli utils
    let components = fs::read_dir(components_path)
        .expect("Source directory should contain sub-directory '/source/components'");

    for component_dir in components {
        let component_dir: DirEntry = component_dir.unwrap();

        // TODO: These might be nice to have
        // Exclude the debugger and disassembler dirs because they give 'undefined type' errors
        if [OsString::from("debugger"), OsString::from("disassembler")]
            .contains(&component_dir.file_name())
        {
            continue;
        }

        for c_file in fs::read_dir(component_dir.path()).unwrap() {
            let c_file: DirEntry = c_file.unwrap();

            // Put all the object files in one output directory regardless of the source directory structure
            // This doesn't lead to file name collisions because all the ACPICA source files are prefixed with the directory they come from anyway
            // E.g. the debugger and disassembler directories both have `names` C files, but one is `dbnames.c` and one is `dmnames.c`
            // so they will end up in different object files.
            let obj_file = {
                let mut c_file_name = c_file.file_name();
                c_file_name.push(".o");
                
                let mut obj_file_path = out_dir.to_path_buf();
                obj_file_path.push(c_file_name);
                obj_file_path
            };

            object_files.push(obj_file.clone());

            if obj_file.exists() {
                continue;
            }

            println!("Compiling {:?}", c_file.path());

            let gcc_command_output = Command::new("gcc")
                .arg("-o")
                .arg(obj_file) // Set output object file
                .arg("-c")
                .arg(c_file.path()) // Set input C file
                .arg({
                    let mut s = OsString::from("-I");
                    s.push(acpica_dir.as_os_str());
                    s.push("/source/include");

                    println!("{s:?}");

                    s
                }) // Set /source/include as directory for header files
                .arg("-DACPI_DEBUG_OUTPUT") // TODO: Get it to compile without this for non-debug builds
                .arg("-fno-stack-protector") // Remove stack protections to get rid of __stack_chk_fail linker warning
                .arg("-g") // Produce debug symbols
                .output()
                .unwrap();

            // Check for compilation errors
            if !gcc_command_output.status.success() {
                eprintln!("Compilation failed:");
                println!(
                    " --- stdout\n{}",
                    String::from_utf8_lossy(&gcc_command_output.stdout)
                );
                println!(
                    " --- stderr\n{}",
                    String::from_utf8_lossy(&gcc_command_output.stderr)
                );
                panic!();
            }
        }
    }

    // Return the generated object files
    object_files
}

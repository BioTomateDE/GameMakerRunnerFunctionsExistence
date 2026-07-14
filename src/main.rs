// See https://discord.com/channels/566861759210586112/568950566122946580/1526322325333737724
//
// BuiltinList.cs was pulled from https://github.com/UnderminersTeam/UndertaleModTool/blob/stable-0.9/UndertaleModLib/Compiler/BuiltinList.cs
use libgm::{
    error::*,
    wad::{GMData, GMVersion, version::LtsBranch},
};

const DATA_PATH: &str = concat!(
    env!("XDG_DATA_HOME"),
    "/Steam/steamapps/common/DELTARUNEdemo/assets/game.unx"
);
const RUNNER_PATH: &str = concat!(
    env!("XDG_DATA_HOME"),
    "/Steam/steamapps/common/DELTARUNEdemo/runner"
);

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1B[31mError: {}\x1B[0m", e.chain_pretty());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("Reading data file");
    let mut data = GMData::default();
    data.general_info.version = GMVersion::new(2022, 1, 2, 0, LtsBranch::Pre2022);
    data.general_info.wad_version = 17;
    println!("Wow parsing was successful");

    let mut funcs: Vec<&str> = extract_builtinlist().ctx("extracting builtinlist")?;

    let mut non_existing_funcs: Vec<&str> = Vec::new();

    println!("Beginning loop of pain");
    loop {
        let mut data: GMData = data.clone();
        for funcname in funcs.clone() {
            data.functions.make(funcname, &mut data.strings);
        }

        println!("Writing modified data file with {} functions", funcs.len());
        libgm::wad::build_file(&data, DATA_PATH)?;

        let out = execute_runner()?;

        // Sample stdout end:
        // Process Chunk: FUNC   126344
        // ERROR!!! :: Error on load
        // Unable to find function @@Other@@
        let needle = "Unable to find function ";
        let Some(idx) = out.find(needle) else {
            println!("{out}");
            bail!("Unrecognized runner output, can't find a 'Unable to find function' error");
        };

        let out: &str = &out[idx + needle.len()..];
        let end: usize = out.find('\n').unwrap_or(out.len());
        let bad_function: &str = out[..end].trim();
        println!("Function {bad_function:?} does not exist apparently");

        let function_index = funcs
            .iter()
            .position(|&f| f == bad_function)
            .ok_or("could not find function in my list")?;
        non_existing_funcs.push(funcs[function_index]);
        funcs.remove(function_index);

        // WARN: file is only accurate when this thing is finished (when game successfully launches)
        std::fs::write("existing_funcs.txt", funcs.join("\n"))
            .ctx_any("could not save existing funcs file")?;

        std::fs::write("nonexisting_funcs.txt", non_existing_funcs.join("\n"))
            .ctx_any("could not save nonexisting funcs file")?;

        // if successful, the runner will keep running forever.
        // then just terminate it and copy back the original data file.
    }
}

fn execute_runner() -> Result<String> {
    println!("Executing runner...");
    for _ in 0..3 {
        // this stdbuf thing fixes output cutoff for piping. dont ask me.
        let output = std::process::Command::new("stdbuf")
            .arg("-o0")
            .arg(RUNNER_PATH)
            .output()
            .ctx_any("executing runner")?;

        let out = String::from_utf8_lossy(&output.stdout);
        if out.contains("Unable to find game") {
            eprintln!("{out}");
            println!("RUNNER IS STUPID!!! Retrying shortly after.");
            std::thread::sleep(std::time::Duration::from_millis(420));
        } else {
            return Ok(out.to_string());
        }
    }
    Err(err!("Runner really can't find the data file :("))
}

fn extract_builtinlist() -> Result<Vec<&'static str>> {
    const RAW_FILE: &str = include_str!("../BuiltinList.cs");
    const NEEDLE: &str = "DefineFunction(\"";
    let mut file: &str = RAW_FILE;
    let mut funcs: Vec<&str> = Vec::new();

    while let Some(idx) = file.find(NEEDLE) {
        file = &file[idx + NEEDLE.len()..];
        let end = file.find('"').ok_or("Quote never closed")?;
        funcs.push(&file[..end]);
        file = &file[end..];
    }

    Ok(funcs)
}

// See https://discord.com/channels/566861759210586112/568950566122946580/1526322325333737724
//
// BuiltinList.cs was pulled from https://github.com/UnderminersTeam/UndertaleModTool/blob/stable-0.9/UndertaleModLib/Compiler/BuiltinList.cs
use libgm::{error::*, wad::GMData};

// The original Undertale data file.
// You should create (copy) this file manually before running the tool.
const ORIGINAL_DATA_PATH: &str = concat!(
    env!("XDG_DATA_HOME"),
    "/Steam/steamapps/common/Undertale/assets/game.unx1"
);
const DATA_PATH: &str = concat!(
    env!("XDG_DATA_HOME"),
    "/Steam/steamapps/common/Undertale/assets/game.unx"
);
const RUNNER_PATH: &str = concat!(
    env!("XDG_DATA_HOME"),
    "/Steam/steamapps/common/Undertale/runner"
);

fn main() {
    if let Err(e) = run() {
        eprintln!("\x1B[31mError: {}\x1B[0m", e.chain_pretty());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    println!("Reading data file");
    let data: GMData = libgm::wad::parse_file(ORIGINAL_DATA_PATH)?;
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

        println!("Executing runner");
        // this stdbuf thing fixes output cutoff for piping. dont ask me.
        let output = std::process::Command::new("stdbuf")
            .arg("-o0")
            .arg(RUNNER_PATH)
            .output()
            .ctx_any("executing runner")?;
        println!("Runner finished (crashed probably)");

        // Sample stdout end:
        // Process Chunk: FUNC   126344
        // ERROR!!! :: Error on load
        // Unable to find function @@Other@@
        let out = String::from_utf8_lossy(&output.stdout);
        let mut lines = out.lines().map(str::trim).filter(|s| !s.is_empty());
        let last = lines.next_back().unwrap_or("");
        let secondlast = lines.next_back().unwrap_or("");

        if !secondlast.starts_with("ERROR!!!") {
            println!("{out:?}");
            println!("\nLast lines:\n{secondlast:?}\n{last:?}");
            bail!("Unrecognized runner output, no 'ERROR!!!' at the end");
        }

        let Some(badfunc) = last.strip_prefix("Unable to find function ") else {
            println!("{out}");
            println!("\nLast lines:\n{secondlast:?}\n{last:?}");
            bail!("Last runner stdout line does seem to be a 'Unable to find function' error");
        };

        println!("Function {badfunc:?} does not exist apparently");

        let badidx = funcs
            .iter()
            .position(|&f| f == badfunc)
            .ok_or("could not find function in my list")?;
        non_existing_funcs.push(funcs[badidx]);
        funcs.remove(badidx);

        // WARN: file is only accurate when this thing is finished (when game successfully launches)
        std::fs::write("existing_funcs.txt", funcs.join("\n"))
            .ctx_any("could not save existing funcs file")?;

        std::fs::write("nonexisting_funcs.txt", non_existing_funcs.join("\n"))
            .ctx_any("could not save nonexisting funcs file")?;

        // if successful, the runner will keep running forever.
        // then just terminate it and copy back the original data file.
    }
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

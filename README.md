This is a small tool to check which functions exist for a given GameMaker runner.
I made it to improve
[`BuiltinList.cs`](https://github.com/UnderminersTeam/UndertaleModTool/blob/stable-0.9/UndertaleModLib/Compiler/BuiltinList.cs)
from [UndertaleModTool](https://github.com/UnderminersTeam/UndertaleModTool)
which currently incorrectly assumes certain builtin GameMaker functions exist for old GMS1 versions.

The main resources here are in the [`output` directory](output/), which contains lists of
existing and missing functions for Linux and Windows runners of various versions.
> (Only functions already defined by BuiltinList are checked for. Perhaps there are more functions that do exist. This tools only checks for false-positives, not false-negatives.)

## Versions

| WAD | GM | Game | Existing Functions File | Missing Functions File |
|-----|----|-----------|-------------------------|-----------------------------|
| 14 | 1.4.1567 | UT 1.00 | [functions-wad14-gm1.4.1567.txt](output/functions-wad14-gm1.4.1567.txt) | [output/missingfunctions-wad14-gm1.4.1567.txt](output/missingfunctions-wad14-gm1.4.1567.txt) |
| 15 | 1.4.1690 | UT 1.001 | [functions-wad15-gm1.4.1690.txt](output/functions-wad15-gm1.4.1690.txt) | [output/missingfunctions-wad15-gm1.4.1690.txt](output/missingfunctions-wad15-gm1.4.1690.txt) |
| 16 | 1.4.1539 | UT 1.06 - 1.08 | [functions-wad16-gm1.4.1539.txt](output/functions-wad16-gm1.4.1539.txt) | [missingfunctions-wad16-gm1.4.1539.txt](output/missingfunctions-wad16-gm1.4.1539.txt) |
| 16 | 2.0.6 | UT 1.09 - 1.11 | [functions-wad16-gm2.0.6.txt](functions-wad16-gm2.0.6.txt) | [missingfunctions-wad16-gm2.0.6.txt](output/missingfunctions-wad16-gm2.0.6.txt)
| 17 | 2.3.2 | DR Demo 1.00 - 1.07 | [functions-wad17-gm2.3.2.txt](output/functions-wad17-gm2.3.2.txt) | [missingfunctions-wad17-gm2.3.2.txt](output/missingfunctions-wad17-gm2.3.2.txt) |
| 17 | 2022.1 | DR Demo 1.08 - 1.10 | [functions-wad17-gm2022.1.txt](output/functions-wad17-gm2022.1.txt) | [missingfunctions-wad17-gm2022.1.txt](output/missingfunctions-wad17-gm2022.1.txt) |

- **WAD** refers to the bytecode/WAD Version field (one byte) in the `GEN8` chunk of the data file (General Info).
- **GM** refers to the IDE Version field (16 bytes) in the `GEN8` chunk. GMS1 versions are stored as `1.0.0.MINOR`, so `1.4.1567` becomes `1.0.0.1567` in GEN8.
- **Game** refers to the Undertale or Deltarune Version (range) that uses this GameMaker version.
The ranges are inclusive on both sides.

Diffs are available in the [`diffs` directory](diffs/).

## Running yourself
If you want, you can run this scuffed tool yourself.
Be aware, this is made for Linux, so you might need to adjust paths in the top of `main.rs` if you're on a different platform.
Also, this tool patches your Undertale data file, so you will need to copy it back manually.

Since builtin functions are determined by the runner, the specific game (datafile) version shouldn't make a difference:
Undertale 1.06 and 1.08 are both WAD 16 (GEN8 1.0.0.1539) and use the same runner, so they have the same exact builtin functions.
For Undertale, [UndertaleVersionSwitcher](https://github.com/Jacky720/UndertaleVersionSwitcher/tree/main/Runner%20Files) provides runners
for each Undertale version and maps them to in-game versions.

## How it works

1. Create a new empty data file
2. Add functions (known from `BuiltinList.cs`) by creating new entries in the `FUNC` chunk.
3. Write the data file
4. Execute the runner
5. If it fails, extract the error message from stdout
  It will be something like:
  ```
  Process Chunk: FUNC   126344
  ERROR!!! :: Error on load
  Unable to find function @@Other@@
  ```
6. Remove that function from the data file
7. Repeat from Step 3

Yes, it's slow. But it works. (You can just use the precomputed values in `output/` anyway).

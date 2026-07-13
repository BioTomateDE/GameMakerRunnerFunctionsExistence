This is a small tool to check which functions exist for a given GameMaker runner.
I made it to improve
[`BuiltinList.cs`](https://github.com/UnderminersTeam/UndertaleModTool/blob/stable-0.9/UndertaleModLib/Compiler/BuiltinList.cs)
from [UndertaleModTool](https://github.com/UnderminersTeam/UndertaleModTool)
which currently incorrectly assumes certain builtin GameMaker functions exist for old GMS1 versions.

The main resources here are in the [`output` directory](output/), which contains lists of
existing and non-existing functions for Linux and Windows runners of various versions.
> (Only functions already defined by BuiltinList are checked for. Perhaps there are more functions that do exist. This tools only checks for false-positives, not false-negatives.)

## Versions

| WAD | GM | Undertale | Existing Functions File | Non-Existing Functions File |
|-----|----|-----------|-------------------------|-----------------------------|
| 14 | 1.0.0.1567 | 1.00 | [existing_functions-wad14-ut1.00.txt]([output/existing_functions-wad14-ut1.00.txt]) | [output/nonexisting_functions-wad14-ut1.00.txt]([output/nonexisting_functions-wad14-ut1.00.txt]) |

**WAD** refers to the bytecode/WAD Version field (one byte) in the `GEN8` chunk (General Info).
**GM** refers to the IDE Version field (16 bytes) in the `GEN8` chunk (General Info). (GMS1 versions are stored weirdly, don't ask me.)
**Undertale** refers to the Undertale Version (range) that uses this GameMaker version.

## Running yourself
If you want, you can run this scuffed tool yourself.
Be aware, this is made for Linux, so you might need to adjust paths in the top of `main.rs` if you're on a different platform.
Also, this tool patches your Undertale data file, so you will need to copy it back manually.

Since builtin functions are determined by the runner, the specific game (datafile) version shouldn't make a difference:
Undertale 1.06 and 1.08 are both WAD 16 (GEN8 1.0.0.1539) and use the same runner, so they have the same exact builtin functions.
For Undertale, [UndertaleVersionSwitcher](https://github.com/Jacky720/UndertaleVersionSwitcher/tree/main/Runner%20Files) provides runners
for each Undertale version and maps them to in-game versions.

## How it works

1. Load original Undertale data file
2. Create new functions (known from `BuiltinList.cs`) by creating new entries in the `FUNC` chunk.
3. Write the modified data file
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

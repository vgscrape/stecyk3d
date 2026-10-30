# Making the substrate

## What it is

The substrate is one file (plus a folder of small files beside it) that holds fixed data a render needs: lookup tables used by procedural textures and noise, a few pictures such as the moon's, a star catalogue, the Tree Lab's leaves, and the preset libraries.

It is made from **your own copy** of Bryce. Stecyk3D does not ship it: that data belongs to Bryce, and nothing in this package contains any of it. The extractor reads your install and writes the substrate on your machine. You do it once.

Without a substrate, a render stops and says what is missing. The program never guesses a value.

## Make it from Blender (the easy way)

1. Open *Edit > Preferences > Add-ons > Stecyk3D*.
2. Fill in **Bryce 3.0 folder**, **Bryce 3.1 folder**, **Bryce 5.0 folder** and/or **Bryce 5.5 folder** with the install folders you have.
3. Press **Make substrate**.

On success an INFO message says where the substrate was written and, per version, what was recognised and how many kept files were taken. On failure nothing is written and the message says exactly why (for example "no Bryce program file found", or "this is Bryce 3D 3.1, not Bryce 3.0: put it in the Bryce 3.1 folder").

The substrate is written to the file named by the `BRYCE_SUBSTRATE` environment variable if that is set, else to the default place below, and is used from then on. There is no field to fill in afterwards.

Name every install you have. Each adds what only it holds: a Bryce 5.0 sky that uses the star catalogue needs a Bryce 5 install (5.0 or 5.5), and a render that needs something your substrate lacks stops and names it. If that happens, add the install and press **Make substrate** again.

## Make it with the command line

```
stecyk3d-extract [--out SUBSTRATE.bin] [--bryce 3.0=FOLDER ...] [FOLDER ...]
```

Example:

```
stecyk3d-extract --out ~/stecyk/substrate.bin --bryce 3.0="/path/to/Bryce 3D 3.0" --bryce 5.0="/path/to/Corel Bryce 5.0" --bryce 5.5="/path/to/DAZ/Bryce 5.5"
```

- `--bryce V=FOLDER` (V is 3.0, 3.1, 5.0 or 5.5) also refuses a folder that holds another version than V. A bare `FOLDER` takes whatever version it holds.
- A folder may be the install itself or hold it up to two levels below it. `Data`/`data` and every other name are matched without regard to letter case.
- The extractor writes `SUBSTRATE.bin` and a folder `SUBSTRATE.files` beside it. Keep them together.
- It writes nothing if any folder is refused. Refusals are printed on standard error and the exit status is 1. On success it prints what it recognised and what it took.
- Without `--out`, it writes `substrate.bin` to the default place: `stecyk3d/substrate.bin` inside your application-data folder (`%APPDATA%` on Windows, `~/Library/Application Support` on macOS, `$XDG_DATA_HOME` or `~/.local/share` on Linux).

## How programs find the substrate

In this order:

1. the `--substrate` option of `stecyk3d-render`, or the add-on's **Substrate file** field under Advanced;
2. the `BRYCE_SUBSTRATE` environment variable;
3. the default place above.

## Bryce 5.5

DAZ Bryce 5.5 (2005; its program is `Bryce55.exe`, its maths library `B55Axiom.dll`) can make the substrate on its own or beside the other installs. The extractor knows the build by size and SHA-256 (`Bryce55.exe` of 5,193,787 bytes) and refuses any other.

**What it takes.** Every table a render as 5.0 needs is in 5.5's files, with the same values as 5.0's: the extractor finds each in 5.5's own files and checks it, so the tables a substrate holds are the same whether they came from 5.0, from 5.5 or from both (a substrate made from both is accepted only because they agree). In 5.5 some of them stand elsewhere: the anti-aliasing offsets are a table of data where 5.0 builds them in code, and the default leaf's points, places and triangles are built by the Tree Lab's code (`CTreeLab::CreateDefaultLeafMesh`) where 5.0 builds them in the tree code. The files kept beside the substrate are 5.5's: its moon picture (the same pixels as 5.0's, in a file that differs only in its container), the star catalogue (the same file as 5.0's), the 61 installed leaves (the same as 5.0's) and its 49 preset libraries (`.mat` and `.brt`; 5.5 brings 41 material libraries in new folders, where 5.0 has 15). They are kept under `DAZ Bryce 5.5` in the folder beside the substrate.

**Rendering as 5.5.** "5.5" is a program choice beside 3.0, 3.1 and 5.0 (the Render tab's program, the description's version, `--as 5.5`). A render as 5.5 uses 5.0's rules, apart from what is known to differ:

- Bryce 5.5 draws the moon's disc, and takes the moon's phase, at half the angle 5.0 does (its sky code multiplies the size by 1.8 and by π/360 where 5.0 multiplies by π/180; the sun's disc is unchanged). A render as 5.5 does the same, so a moon in view is half the size it is as 5.0; everything else in the picture is 5.0's.

The preset libraries offered for 5.5 are 5.5's own first, then the other Bryce 5 installs'.

How this was found: by comparing the two programs' code, function by function, with the debug symbols 5.5 ships (`Bryce55.pdb`). 5.5 was built with other compiler settings, so the comparison is of the arithmetic each function does and the constants it reads, not of bytes; it covered the renderer's sources (sky, textures and noise, materials, rays, objects, trees, terrain) but was not complete, and 5.5's pictures have not been compared with Bryce 5.5's own. Scene documents (`.br5`) and preset libraries of 5.5 are read as 5.0's; two of 5.5's material libraries (`Waters/Calm.mat` and `Waters/Turbulent.mat`) are stored in the blank-led form of Bryce 5's documents and are not read yet.

## If the file is refused

A substrate with a wrong checksum, an old format or a bad magic number is refused whole. Make it again with the current `stecyk3d-extract`.


![WICS](./assets/RepoHeader.png)
<sub>/'wɪks/</sub>

# READ BOLD PART OF NOTE!!!

> [!NOTE]
> Read carefully since this is kinda complicated xd:
>
> This branch is dedicated for well big update (almost a rewrite honestly) of the rewrite of wics. This branch will be in rust same as 2.0.0 one. Why a separate branch you might ask, I think its better to keep old one since it worked partially and I have no clue if this approach will work at all. Anyway, **If you want a functional app just go to main branch and install it using instructions in it's `README.md`, it works well**, both -rust branches are here since I just want to experiment and check if I can come up with something that works better. IF you are still reading you are likely for some reason interested in what I do here so here is general idea of what I want to change in this branch with some more technical details: As it turned out modId usually doesn't reflect slug of modrinth's project (who would've thought (well I didn't)) so shitload of mods just throw 404 which makes 2.0.0 (well v2.0.1 release) not worth using. So now I want to first calculate sha1 or sha2 of a .jar then provide it to modrinth api to get thier internal project_id and then query for that internal id, shoutout to Gemini for that idea since I had no clue for another approach to that project and I have no friends to think this through with. Will this work? I have no clue (like rly 0, I don't even know if it's really possible to query modrinth api with file hash, didn't check yet) but I'll try and maybe it will be best app in the world? Might be if you ask me.
>
> This means most od readme.md following this point will not be accurate since i didn't care to update it. I will sometime.

**What Is Client Sided** is a little script that checks which files in a specified directory are client-sided Minecraft mods - super handy for figuring out which mods shouldn't be put on a server ♡
<br>

<br><br>

# 🗺️ Roadmap


| <img src="./assets/check.gif" width="15px">   	| First stable release                                                            	|
|-----------------------------------------------	|:--------------------------------------------------------------------------------	|
| <img src="./assets/loading.gif" width="15px"> 	| Add help argument to the command                                                	|
| <img src="./assets/loading.gif" width="15px"> 	| ¹ Modrinth API support (checking mod sides via Modrinth instead of local files) 	|

*¹ i don't know if I'll actually do this one*

<br><br>

# 📦 Installation

## Windows

### Automatic ✨

Download the repo either via `git clone https://github.com/TaXi0k/WICS` or by pressing the green `Code` button above and selecting `Download ZIP` - then just run `INSTALL.bat` and follow the instructions in there!

### Manual 🔧

**1. Download WICS**<br>
Either use `git clone https://github.com/TaXi0k/WICS` or download manually via the `Code` button!

**2. Install Node.js**<br>
WICS needs Node.js to function at all - if you haven't installed it yet, now's the time! :3

> [!TIP]
> There are multiple ways of installing node, the simples beeing via winget: `winget install -e --id OpenJS.NodeJS.LTS`

**3. Actually install WICS**<br>
Navigate to `WICS/WICS/` directory (the one containing `src/`) in your terminal and run:

``` bat
npm install
npm run build
npm link
```

> [!WARNING]
> Global linking (`npm link`) might need admin privileges - but don't worry, WICS will still work without it! Just run it as `node <path-to-index.js> <path-to-directory>` instead ♡

## Linux

1. Clone this repo
2. Install Node.js if you don't have it already
3. Navigate to `WICS/WICS/` (where you have `src/`) and run following commands:
``` sh
npm install
npm run build
npm link
```

<br><br>

# 🗑️ Uninstallation

1. Navigate to `WICS/WICS/` in your terminal and run `npm unling -g`.
2. Now you're safe to delete the project folder!

> [!NOTE]
> If `npm link` never succeeded during installation, step 1 will likely throw a harmless error - just skip it and go straight to deleting the folder ♡

<br><br>

# 🚀 Usage

Using WICS is unbelievably simple! Just run:

``` bat
wics <path>
```

where `<path>` is the path to the directory you want to scan - both absolute and relative paths work. That's really it! ★

> [!WARNING]
> If `npm link` failed during installation, use `node <path-to-index.js> <path-to-directory>` instead - again, both absolute and relative paths are fine!

<br><br>

# ❌ Unsupported mods

Basically every mod that doesn't contain a standard metadata files (`mods.toml` / `neoforge.mods.toml` / `fabric.mod.json`).

Here is the list of mods that I know are unsupported (just skipped by the script):
<details>
<summary><b>List</b></summary>

* Connector
* Kotlin for forge

</details>

<br><br>

# 🤝 Contributing

Contributions are always welcome and mean a lot! ♡ Here's how you can help:

* 🪱 **Found a bug?** Open an issue!
* 🧠 **Got an idea?** Open an issue and let's talk about it!
* 🩷 **Want to fix or add something?** PRs are very welcome - just describe what you changed and why ★

> [!NOTE]
> Please, submit PRs and issues only in **English** (or **Polish** if you prefer).

<br><br>

# 📄 License

**WICS** is shared under the **MIT License** - see [LICENSE](LICENSE) for details.

> [!NOTE]
> The license is all that's legally required, but if you're able to,
> a little visible credit means the world to me - a mention in your README,
> about page, or anywhere that fits ♡ Linking back to this repo would be
> amazing too, but no pressure at all!

### 💗 TYSM for using WICS, I hope it treats you well ★


# WICS ★

**What Is Client Sided** (WICS for short) is a little script that checks which files in a specified directory are client-sided Minecraft mods - super handy for figuring out which mods shouldn't be put on a server ♡**

# 🗺️ Roadmap


| <img src="./assets/check.gif" width="15px">   	| First stable release                                                            	|
|-----------------------------------------------	|:--------------------------------------------------------------------------------	|
| <img src="./assets/loading.gif" width="15px"> 	| Add help argument to the command                                                	|
| <img src="./assets/loading.gif" width="15px"> 	| ¹ Modrinth API support (checking mod sides via Modrinth instead of local files) 	|

*¹ i don't know if I'll actually do this one*

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

Coming soon!

# 🗑️ Uninstallation

1. Navigate to `WICS/WICS/` in your terminal and run `npm unling -g`.
2. Now you're safe to delete the project folder!

> [!NOTE]
> If `npm link` never succeeded during installation, step 1 will likely throw a harmless error - just skip it and go straight to deleting the folder ♡

# 🚀 Usage

Using WICS is unbelievably simple! Just run:

``` bat
wics <path>
```

where `<path>` is the path to the directory you want to scan - both absolute and relative paths work. That's really it! ★

> [!WARNING]
> If `npm link` failed during installation, use `node <path-to-index.js> <path-to-directory>` instead - again, both absolute and relative paths are fine!

# 🤝 Contributing

Contributions are always welcome and mean a lot! ♡ Here's how you can help:

* 🪱 **Found a bug?** Open an issue!
* 🧠 **Got an idea?** Open an issue and let's talk about it!
* 🩷 **Want to fix or add something?** PRs are very welcome - just describe what you changed and why ★

> [!NOTE]
> Please, submit PRs and issues only in **English** (or **Polish** if you prefer).

# 📄 License

**WICS** is shared under the **MIT License** - see [LICENSE](LICENSE) for details.

> [!NOTE]
> The license is all that's legally required, but if you're able to,
> a little visible credit means the world to me - a mention in your README,
> about page, or anywhere that fits ♡ Linking back to this repo would be
> amazing too, but no pressure at all!

### TYSM for using WICS, I hope it treats you well ★

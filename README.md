<p align="center">
</p>

<div align="center">
<img src="https://github.com/golota60/idasen-tray-controller/blob/master/public/carrot.png" width="200">
	<h1>Idasen controller</h1>
	<p>
		<b>🥕 A cross-platform background tray app for controlling your IKEA Idasen desk 🥕</b>
	</p>
	<br>
</div>

|                                       Linux                                        |                                       MacOS                                        |                                     Windows                                      |
| :--------------------------------------------------------------------------------: | :--------------------------------------------------------------------------------: | :------------------------------------------------------------------------------: |
| ![](https://github.com/golota60/idasen-tray-controller/blob/master/linux-demo.png) | ![](https://github.com/golota60/idasen-tray-controller/blob/master/macos-demo.png) | ![](https://github.com/golota60/idasen-tray-controller/blob/master/win-demo.png) |

<br>

## Installation

[Here](https://github.com/golota60/idasen-tray-controller/releases/) you can find all the releases and associated files you should download.

## Usage

When you open the app for the first time, it will open up a setup screen, and display all the bluetooth devices with "Desk" in their name. Once connected, the desk name will be saved and it will not appear when opening the app later.

After first usage, the name of the desk is saved in the configuration file, so the next time you open the app, it should connect to the desk automatically.

The desk cannot be connected to multiple machines at once, so make sure the desk is not connected to anything when you open the app.

## System-specific quirks

Some systems can handle the app gracefully, some don't - here are the quirks i've found while using on different systems

### Windows

From my experience, in order to connect to the desk I had to first connect the desk via bluetooth control panel. After that, the app can pick up on it.

### MacOS

Desk should NOT be connected to the system while opening the app. This is a weird quirk of MacOS that I'll look into fixing in the future. Also, since I'm not a signed Apple developer, you might get a prompt saying that the app can be opened - [here's how to bypass that](https://support.apple.com/en-us/HT202491).

### Linux

Connect to the desk using your system bluetooth control. After this, everything should just work. But since there are a lot of linux flavors out there, there's a chance that your machine might have some different quirks.

## Troubleshooting and config resetting

If you want to reset your config go into `About/Options` menu and you should see a config reset button.
If you encounter any problems that were not explained anywhere in this README, feel free to open an issue describing your problem. If you wish to inspect the config file, below are the locations.

- MacOS

```bash
 /Users/<your_profile_name>/Library/Application\ Support/idasen-tray-config.json
```

- Linux

```bash
 /home/<your_profile_name>/.local/share/idasen-tray-config.json
```

- Windows

```bash
 C:\\Users\\<your_profile_name>\\AppData\\Roaming/idasen-tray-config.json
```

**Important** - If you changed your desk device name, you'll also need to restet the config.

## Self-compiling

If there's no build for your particular machine, feel free to clone the repo and self-compile according to tauri docs
https://tauri.app/v1/guides/building/

## Developing

Prerequisites are `node`, `yarn` and `rust`.

To run the app in developer environment, clone the repo, run `yarn` in the root(to install JS dependencies), and then run `yarn tauri dev`. The app might take a while to build for the first time.

## Roadmap, known issues and feature requests

Roadmap(w/o order):

- [x] Create config file if not present
- [x] Drop MAC address requirement; add some better way - replaced with device name
- [x] Auto save MAC address upon first connection to be reused later
- [x] Adding new desk positions
- [x] Deleting desk positions
- [x] Windows support
- [x] MacOS support
- [x] Nicer icon
- [x] Better input window decorator- no need imo
- [x] Better desk moving behavior(currently it moves weirdly, due to usage of external lib)
- [x] More information inside README + potential problems
- [ ] Better tests
- [ ] Run on system startup
- [x] Display a setup screen instead of automatic connection for better UX
- [x] Allow for config reset from inside the app
- [ ] Add options(TBD which ones)
- [ ] Automatic update prompts(?)
- [ ] Automatic deploys & changelog on CI

Known issues:

- [x] Clicking on newly added element
- [ ] manually stopping a moving desk deadlocks the app
- [ ] opening a new window while the other is already open

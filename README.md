<p align="center">
</p>

<div align="center">
<img src="https://github.com/golota60/trayasen/blob/master/public/carrot.png" width="200">
	<h1>Trayasen</h1>
	<p>
		<b>🥕 A cross-platform background tray app for controlling your IKEA Idasen desk 🥕</b>
	</p>
	<br>
</div>

|                                Linux                                 |                                MacOS                                 |                              Windows                               |
| :------------------------------------------------------------------: | :------------------------------------------------------------------: | :----------------------------------------------------------------: |
| ![](https://github.com/golota60/trayasen/blob/master/linux-demo.png) | ![](https://github.com/golota60/trayasen/blob/master/macos-demo.png) | ![](https://github.com/golota60/trayasen/blob/master/win-demo.png) |

<br>

## Installation

[Here](https://github.com/golota60/trayasen/releases/) you can find all the releases and associated files you should download.

## Usage

When you open the app for the first time, it will open up a setup screen, and display all the bluetooth devices with "Desk" in their name. Once connected, the desk name will be saved and it will not appear when opening the app later.

After first usage, the name of the desk is saved in the configuration file, so the next time you open the app, it should connect to the desk automatically.

The desk cannot be connected to multiple machines at once, so make sure the desk is not connected to anything when you open the app.

## System-specific quirks

Some systems can handle the app gracefully, some don't - here are the quirks i've found while using on different systems

### Windows

From my experience, in order to connect to the desk I had to first connect the desk via bluetooth control panel. After that, the app can pick up on it. If it's your first time connecting your desk to your PC, you should get a pop-up on lower right "Click to set up "Desk XXXX". Click on that and then click "Allow", otherwise the app will not be able to connect to your desk.

### MacOS

Desk should NOT be connected to the system while opening the app. This is a weird quirk of MacOS that I'll look into fixing in the future. Also, since I'm not a signed Apple developer, you might get a prompt saying that the app can be opened - [here's how to bypass that](https://support.apple.com/en-us/HT202491).

### Linux

Connect to the desk using your system bluetooth control. After this, everything should just work. But since there are a lot of linux flavors out there, there's a chance that your machine might have some different quirks.

## Troubleshooting and config resetting

If you want to reset your config go into `About/Options` menu and you should see a config reset button. In case you cannot do that, delete the configuration file, path of which you can find below.


If you encounter any problems that were not explained anywhere in this README, feel free to open an issue describing your problem. If you wish to inspect the config file, below are the locations for every system.

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

Frontend is using [shadcn/ui](https://ui.shadcn.com/) for styling 

## Releasing

The release is automatically triggered upon push to the `release` branch. In order to release, simply create commit that bumps up version in `tauri.conf.json` in master, then create a `release` branch that mirrors `master` branch. After the pipeline finishes - there should be a draft release create it. Then simply navigate to "Releases" and release the draft.

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
- [x] Run on system startup
- [x] Display a setup screen instead of automatic connection for better UX
- [x] Allow for config reset from inside the app
- [ ] Add options(low/high perf mode, system startup toggle)
- [ ] ~Automatic update prompts(?)~ who would want their desk manager app to connect to the internet
- [x] Automatic deploys
- [ ] low/high perf mode

Known issues(checked means fixed):

- [x] Clicking on newly added element
- [ ] Manually stopping a moving desk deadlocks the app
- [x] Opening a new window while the other is already open
- [ ] Race condition when moving the desk right after opening the app
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

Desk should NOT be connected to the system while opening the app. This is a weird quirk of MacOS that I'll look into fixing in the future. Also, since I'm not a signed Apple developer, you might get a prompt saying that the app can be opened - [here's how to bypass that](https://apple.stackexchange.com/a/267555).

### Linux

Connect to the desk using your system bluetooth control. After this, everything should just work. But since there are a lot of linux flavors out there, there's a chance that your machine might have some different quirks.

## Resetting to factory settings

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

**Important** - If you changed your desk device name, you'll also need to reset the config.

## Troubleshooting

Here are the errors that might happen to you. If you don't see your error(or a fix to it doesn't work) described below, please open an issue, along with the reproduction steps:

- `Cannot subscribe to read position`

On Windows, this means that this is the first time your computer is connecting to your desk, and has not been set up properly yet.

Fix 1: 
1. While connecting to the desk inside the app(essentially, clicking "Connect" button), you should see a windows popup(in the lower right corner of your screen) with the text saying - "Tap to set up Desk XXXX". 
2. Click on that, and select "Allow". 
3. After that, click "Reset config & restart the app" in the Trayasen app. After this, the next connection to your desk should be successful.

Fix 2:
1. In bluetooth settings, set the following option to "Advanced" 
<img src="https://github.com/golota60/trayasen/blob/master/win-bluetooth.png">

1. Next, try adding a bluetooth device. You should see your desk with the name `Desk XXXX`. Connect to it. After successful connection, you should be able to connect to it from Trayasen. 

- `Desk connection not initiated`

This is a signal from an application, that we're trying to perform a bluetooth operation without being connected to a desk. That might mean that we're trying to move the desk without the setup step. If you encounter this, please open an issue with reproduction steps.

- `Unknown error while connecting`

This usually means that the desk you're trying to connect to, is refusing the connection. Usually, the most common fix is to hold down the bluetooth button you have on the desk handle, until the blue light starts flashing, and then retrying the connection.

- `Cannot find the device` 

This usually means, that your saved desk is not visible to your PC's bluetooth. Usually, the most common fix is to hold down the bluetooth button you have on the desk handle, until the blue light starts flashing, and then retrying the connection.

- `Cannot subscribe to read position`

Means that the bluetooth connection with your desk was successful, but for some reason your PC has problems talking to it. Usually, the most common fix is to hold down the bluetooth button you have on the desk handle, until the blue light starts flashing, and then retrying the connection.

- `bltleplug error: <content>` 

This means the applications bluetooth library encountered something it wasn't expecting. If you encounter this, please create an issue with your system and reproduction steps


- Shortcut not working

Shortcuts are a pain, and we're limited by the upstream framework we're using. See [this](https://github.com/golota60/trayasen/issues/20#issuecomment-1868359329), [this](https://github.com/golota60/trayasen/issues/20#issuecomment-1725195092) and [this](https://github.com/golota60/trayasen/issues/16#issuecomment-1724970876) for context.

If you cannot register a shortcut you want, please advise the comments above. You essentially need to manually open the config file and try different key combinations.


## Self-compiling

If there's no build for your particular machine, feel free to clone the repo and self-compile according to tauri docs
https://tauri.app/v1/guides/building/

## Developing

Prerequisites are `node`, `yarn` and `rust`.

To run the app in developer environment, clone the repo, run `yarn` in the root(to install JS dependencies), and then run `yarn tauri dev`. The app might take a while to build for the first time.

Frontend is using [shadcn/ui](https://ui.shadcn.com/) for styling 

## Releasing

The release is automatically triggered upon push to the `release` branch. In order to release, simply create commit that bumps up version in `tauri.conf.json`/`Cargo.toml`/`package.json` in master, then create a `release` branch that mirrors `master` branch. After the pipeline finishes - there should be a draft release create it. Then simply navigate to "Releases" and release the draft.

## Roadmap, known issues and feature requests

Roadmap and issues are tracked in the [issues](https://github.com/golota60/trayasen/issues) and [project tracker](https://github.com/users/golota60/projects/3) respectively.
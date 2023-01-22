currently app needs an exact MAC address to work

needed to work
https://github.com/tauri-apps/tao#linux

dirs reference:
https://tauri.app/v1/api/js/path#datadir

on linux config file is in:
$HOME/.local/share/idasen-tray/config.json

macos brew deps to compile:
pkg-config
cairo
gdk-pixbuf
pango
atk
gtk+3

roadmap:

- [x] create config file if not present
- [x] drop MAC address requirement; add some better way
- [x] auto save MAC address upon first connection to be reused later
- [x] implement adding new/deleting old desk positions
- [ ] windows support
- [ ] macos support
- [x] nicer icon
- [ ] hover text + window title text
- [ ] better desk moving behavior(currently it moves weirdly)
- [ ] more information inside README(looks, potential problems, manual config setting, moving config from pc to pc)
- [ ] app tests
- [ ] research dropping tao for GTK usage only
- [ ] run on system startup(?)

overall bugs:
- [ ] clicking on newly added element

macos bugs:

- [ ] build
- [ ] problems with connecting - need to reconnect every time - maybe fixing below will fix this
- [ ] MAC address is 00:00 always for some reason
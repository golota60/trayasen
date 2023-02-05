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
- [x] adding new desk positions
- [x] deleting desk positions
- [ ] windows support
- [ ] macos support
- [x] nicer icon
- [x] better input window decorator- no need imo
- [ ] better desk moving behavior(currently it moves weirdly, due to usage of external lib)
- [ ] more information inside README(looks, potential problems, manual config setting, moving config from pc to pc)
- [ ] app tests
- [ ] run on system startup(?)
- [ ] new screen with bluetooth selection

overall bugs:

- [x] clicking on newly added element
- [ ] manually stopping a moving desk deadlocks the app
- [ ] opening a new window while the other is already open
- [ ] routing is fucked on client side

macos bugs:

- [ ] build
- [ ] MAC address is 00:00 always for some reason
- [ ] prod - MACOS - cannot exit window on ready

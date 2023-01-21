currently app needs an exact MAC address to work

needed to work
https://github.com/tauri-apps/tao#linux

dirs reference:
https://tauri.app/v1/api/js/path#datadir

on linux config file is in:
$HOME/.local/share/idasen-tray/config.json

roadmap:

- [x] create config file if not present
- [ ] implement adding new profiles
- [x] drop MAC address requirement; add some better way
- [ ] windows support
- [ ] macos support
- [ ] nicer icon
- [ ] better desk moving behavior(currently it moves weirdly)
- [ ] run on system startup(?)
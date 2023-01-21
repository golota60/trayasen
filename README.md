currently app needs an exact MAC address to work

needed to work
https://github.com/tauri-apps/tao#linux

dirs reference:
https://tauri.app/v1/api/js/path#datadir

on linux config file is in:
$HOME/.local/share/idasen-tray/config.json

roadmap:

- [x] create config file if not present
- [x] drop MAC address requirement; add some better way
- [x] auto save MAC address upon first connection to be reused later
- [ ] implement adding new desk positions
- [ ] windows support
- [ ] macos support
- [x] nicer icon
- [ ] better desk moving behavior(currently it moves weirdly)
- [ ] more information inside README(looks, potential problems, manual config setting, moving config from pc to pc)
- [ ] app tests
- [ ] run on system startup(?)
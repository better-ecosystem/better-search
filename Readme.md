# Better-Search
A blazing fast rust based app,files,web search &amp; launch tool


## Install

Dependencies

| OS | Required Packages |
|---------|------------------|
| Arch/Arch-based | `sudo pacman -S xdg-utils` |
| Gentoo/Gentoo-based | `sudo emerge xdg-utils` |
| Debian/Debian-based | `sudo apt install xdg-utils` |
| Fedora/Fedora-based | `sudo dnf install xdg-utils` |
| Void | `sudo xbps-install -S xdg-utils` |
| OpenSUSE | `sudo zypper install xdg-utils` |
| Alpine | `sudo apk add xdg-utils` |


> [!IMPORTANT]
> 🚧 Get dependencies according to your system before install and manual build will take time and requires you to have rust installed


<details>
<summary><b>Manual Build</b></summary>
  
```
  git clone https://github.com/better-ecosystem/better-search
  cd better-search
  make install
```

</details>
<details>
<summary><b>Quick Install</b></summary>

  ```
curl -L https://github.com/better-ecosystem/better-search/releases/download/v1.0.0/search -o ~/.local/bin/search && chmod +x ~/.local/bin/search
```

</details>

---

## Usage
- You can launch the app with the `search` command or bind a key to open it with the same command for quick use

---

## Uninstall
```
rm -rf ~/.local/bin/search
```


### Made with ❤️ for the Linux community

[Report Bug](https://github.com/better-ecosystem/better-search/issues) •
[Request Feature](https://github.com/better-ecosystem/better-search/discussions) 


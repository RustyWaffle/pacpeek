# pacpeek

Pacpeek is a small CLI tool, that is completely unnecessary, its just way cooler than 'pacman -Qi'.

It's compatible on every OS, ye i'm not joking, try it out!

## Installation

### AUR
```bash
yay -S pacpeek
```
(or your preferred AUR helper — paru, etc.)

### From source (GitHub)
```bash
git clone https://github.com/RustyWaffle/pacpeek
cd pacpeek
cargo build --release
sudo cp target/release/pacpeek /usr/local/bin/
```
## Usage

```
$ pacpeek firefox
_________________________________
NAME: firefox
VERSION: 152.0.6-1
BASE: firefox
DESC: Fast, Private & Safe Web Browser
URL: https://www.mozilla.org/firefox/
ARCH: x86_64
BUILDDATE: 2026-07-14
INSTALLDATE: 2026-07-15
PACKAGER: Christian Hesse <eworm@archlinux.org>
SIZE: 286.5 MB
LICENSE: MPL-2.0
VALIDATION: pgp
DEPENDS: alsa-lib
OPTDEPENDS: hunspell-en_US: Spell checking, American English
XDATA: pkgtype=pkg
_________________________________
```

## Why?

Well, its actually way quicker than 'pacman -Qi', pacpeek finishes in 0.002 sec, crazy right?


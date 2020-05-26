# Aldebaran-rs

Fire Emblem Three Houses (1.2.0) file replacement plugin for Nintendo Switch.

## Prerequisites

* A Nintendo Switch capable of running a Custom Firmware
* The latest release of [Atmosphere CFW](https://github.com/Atmosphere-NX/Atmosphere/releases) by [SciresM](https://github.com/SciresM)
* (OPTIONAL) A TCP client of your choice if you want runtime logs.

## Setup

1. Download the latest [release](https://github.com/three-houses-research-team/aldebaran-rs/releases/latest) of Aldebaran-rs and extract the content of the ``sd`` directory at the root of your SD card.
2. Edit your system settings (atmosphere/config/system_settings.ini) and make sure to edit the ``ease_nro_restriction`` line so that it is toggled on. (``ease_nro_restriction = u8!0x1``)

## Usage

Place the files you edited in ``sd:/Aldebaran/forge`` on your SD card and give them a name representing the FileID of the one you are replacing, WITHOUT EXTENSION.

If you wish to replace the file 0 of DATA1, then the file should be ``sd:/Aldebaran/forge/0``.

NOTE: Filenames are allowed, as long as the FileID is the first thing in the name and the separation is a ``-`` symbol.  
Example: ``sd:/Aldebaran/forge/0-msgdata.bin``

## Extras
- Message logging is available using a TCP client (listen on port 6969 before running the game)

## Credits
* [Raytwo](https://github.com/Raytwo)/[kolakcc](https://github.com/kolakcc) - Aldebaran-rs maintainers, KTGL reverse engineering
* [jam1garner](https://github.com/jam1garner) - [cargo-skyline](https://github.com/jam1garner/cargo-skyline), [skyline-rs](https://github.com/ultimate-research/skyline-rs), [skyline-rs-template](https://github.com/ultimate-research/skyline-rs-template), support and using me as a guinea pig
* [shadowninja108](https://github.com/shadowninja108) and the Skyline contributors - [Skyline](https://github.com/shadowninja108/Skyline)

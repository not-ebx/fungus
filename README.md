# Fungus
Fungus is a Backend emulator for certain Korean Mushroom 2D Side-scrolling MMORPG.
It's currently in-development, written in Rust with tokio.

It's based on the v111 version of the game, and it's not ready whatsoever to be used in "production". Though, you shouldn't use this to host a private server, since it's only used to LEARN!

## Architecture
It follows a non-blocking i/o loop for each server (Login, Channel). This still is not set in stone, and I have a couple ideas I wanna try before picking one.
Current idea is to have an event loop on each channel, but maybe not.

It uses NX files to read the game data, using my own implementation of the PKGNX4 format. You can find it here: https://github.com/not-ebx/rustNX

No MySQL, we use PostgreSQL here.

Also, things are separated in services, so it's easier to share things around between cargos.

## What is currently working?
As of now, not much. Right now, you can join up to the character selection screen, but with no characters at all. I'm working on this source all the time so things will be changing, but as the time of writting this, that's what's working.
I'm always pushing changes to the development branch, maybe I will be updating that README more, but who knows. I'll probably update the README when a big change happens, so keep an eye if you are interested.

## Contributing
I'm not accepting any contributions as of now, because it's still in early development. I will be thinking about it once i reach a stage where the game is at least playable. 
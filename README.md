# Fungus
Fungus is a Backend emulator for certain Korean Mushroom 2D Side-scrolling MMORPG.
It's currently in-development, written in Rust with tokio.

It's based on the v95 version of the game, and it's not ready whatsoever to be used in "production". Though, you shouldn't use this to host a private server, since it's only used to LEARN!

## Architecture
It follows a non-blocking i/o loop for each server (Login, Channel). This still is not set in stone, and I have a couple ideas I wanna try before picking one.
Current idea is to have an event loop on each channel, but maybe not.

It uses NX files to read the game data, using my own implementation of the PKGNX4 format. You can find it here: https://github.com/not-ebx/rustNX

No MySQL, we use PostgreSQL here.

Also, things are separated in services, so it's easier to share things around between cargos.

## What is currently working?
Right now, login happy path is working, up to character creation. No character deletion, and you can't select characters to enter the game. But the happy path is done.

## Contributing
I'm not accepting any contributions as of now, because it's still in early development. I will be thinking about it once i reach a stage where the game is at least playable. 

## Acknowledgements
I did a lot of research in order to develop this piece of software. I thank a lot of the people that worked on projects like these in the past.
- Swordie: The first source I've ever played around with. Taught me a lot how maple works and also how servers work. Managed to downgrade that to v111 (not fully).
- HeavenMS/Cosmic: Another decent resource to learn from
- MinimumDelta: AMAZING learning tutorials for IDA & Client Reverse engineering. Taught me everything I needed!
- Hendi48: Great guy, helped me a lot in understanding some packets. Also great inspiration for me!!
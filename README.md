# Orpheus

## THIS IS A RANT PROJECT

This is a stupid project that shouldn't exist, nonetheless it does.

Why shouldn't it exist? Easy, it shouldn't because huge companies like JBL/Harman should make products that are actually good for listening audio. Though some times they fuck up.

Quite some time ago I bought a [JBL Bar 3.1][jbl-bar], right away I found out it has an EXTREMELY annoying issue. When there's is no digital audio playing it goes into a sort of "standby", and then comes back when there's audio being played. BUT NOT RIGHT AWAY! So the audio is cut. Watching a movie, listening to music, playing videogames and basically anything that requires audio is extremely frustating. Basically it fails at its only job. Analog via AUX has no issue though.

You can find ton of people with the same identical problem with a quick search about the JBL Bar 3.1 sound cutting off. They all failed miserably to solve the problem. Customer support doesn't help either, I can attest to that as I tried prying from their hands an updated firmware that solves the issue. They'll tell you try this or that configuration, that your TV might not be compatible, that you're connecting it wrong, to update it to latest firmware. The usual tech support stuff. I even asked if I could get in touch with an engineer but to no avail. I declared myself defeated after a long thread with them. In the end you're stuck with a flawed product from a company worth billions, owned by another company worth billions, that is in turn owned by yet another company worth billions. I'm talking of JBL, Harman and in the end Samsung.

This project kinda tries to work around the issue by playing extremely low frequency sound that is high volume enough that the bar doesn't go into that "standby" mode. This is obviously not a solution but at least if I'm using my custom media center I can enjoy the audio quite nicely.

In the end I strongly suggest you don't buy any soundbar from JBL/Harman, and always try them live before buying from other brands. It wouldn't surprise me if some other company falls into this pit. Don't buy Samsung either, they own Harman.

# Usage

The Rust code doesn't do anything too fancy, it's been adapted from the [`alsa` library example][alsa-lib-example].

The binary is meant to be used as a systemd user service.

Just run `setup.sh` to build the project, copy the binary and the service definition where the system can find it, and enable and start the service.

After that the service will start every time you login with the user that ran the script. No superuser authorization required.

If you want to disable everything and clean things up just run `teardown.sh`.

# License

The project is licensed under the [AGPLv3](https://www.gnu.org/licenses/agpl-3.0.en.html) license.

[jbl-bar]: https://mm.jbl.com/soundbars/JBL+BAR+3.1.html
[alsa-lib-example]: https://docs.rs/alsa/latest/alsa/pcm/index.html#example

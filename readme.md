# Hatchery

A program to backup your Last.fm data. Thank god for the hatchery, without it
we'd all be lost.

## Usage

``` console
hatchery 0.1.0

Jake Ledoux (contactjakeledoux@gmail.com)

USAGE:
    hatchery [OPTIONS] --api-key <API_KEY> --api-secret <API_SECRET> <USERNAME>

ARGS:
    <USERNAME>    [env: LASTFM_USERNAME=]

OPTIONS:
        --api-key <API_KEY>          [env: LASTFM_API_KEY=]
        --api-secret <API_SECRET>    [env: LASTFM_API_SECRET=]
    -f, --format <FORMAT>            [default: json] [possible values: json, sql]
    -h, --help                       Print help information
    -V, --version                    Print version information
```

## Why?

You learn all sorts of things working for the hatchery, one of those being the
old adage, "Don't put all your eggs in one basket."

Last.fm doesn't seem to be going anywhere quite yet, but the lack of funding and
support its received from its parent company CBS doesn't instill confidence in
the future of the service. Whether you think it will even exist a decade from
now is your opinion, but I don't want to leave my data's safety to chance, so
that's what this is for. It allows you to back up your scrobbles, loved tracks,
and friends to a local database to ensure their safe-keeping in the event that
anything should happen to Last.fm.

*Last.fm is dead, long live Last.fm.*

## AFAQ

Anticipated Frequently Asked Questions:

### Why do you fetch my entire scrobble history every time?

**Short answer:** Scrobble editing/deleting.

If it weren't for that I could just ask
for the scrobbles since the last update and the whole thing would run much
faster. A full backup is what we want, so a full report must be requested
every time.

### Why do you need my secret key?

**Short answer:** I don't.

I only require it in case it's needed in a future version.

### Why can't I export to multiple formats simultaneously?

**Short answer:** Because there's no point.

This isn't something that can't be done, I just don't see a use case where you'd
want both versions. This program is for backing up your data, not preparing it
for data analysis or anything else, and there's no sense backing it up in two
different formats.

### Why can't I back up multiple accounts?

**Short answer:** You can: just run the program multiple times.

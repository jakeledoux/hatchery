# Hatchery

A program to backup your Last.fm data.

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
        --database <DATABASE>        [default: lastfm.db]
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
that's what this is for. It allows you to back up your scrobbles to a local
database to ensure their safe-keeping in the event that anything should happen
to Last.fm.

*Last.fm is dead, long live Last.fm.*

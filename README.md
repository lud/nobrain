# nobrain

This is a small command line utility to (re)generate passwords based on a master
key, a domain name (or any other "domain" identifier), and optionnaly a
username.

With the single master key to remember, you can regenerate the same password
from the same inputs (domain and username) so you do not have to remember them.

The master key is not stored anywhere (only in your brain, so you need to have a
little bit of it after all), not passed as a command argument and never
displayed so it will not show in bash history, logs, etc.

Generated passwords contains letters, capitals, digits and special characters.
We use a custom encoding on top of the SHA1 hash to provide those features.


## Disclaimer

This is a work in progress. It works, but the algorithm used to hash the
password must be changed if favor of a slow one that supports multiple
iterations.

We would also determine a number of iterations from the master key to make
virtually impossible to use brute-force to find the hash.

## Examples

#### Generate a password for example.com

```
$ nobrain example.com
Master key: [hidden]
Domain         example.com
Your password  GxHeUxFeVPU3JPguH$ExVSQvUfUxJDhdHPheUxHeGxcwUiEwHDg1Hc
```

Of course `example.com` is just an arbitrary identifier. It is a good practice
to use the domain name of the website you need a password for, so you do not
need too remember if you used `example`, `Example` or `"Example Site"` …

But any of those will work, too.


#### Generate a password with a username

Simply add the `-u <username>` option.

```
$ nobrain example.com -u lud
Master key: [hidden]
Domain         example.com
Username       lud
Your password  JDcuVi:vGPAwVSUvViHgU0:3GxGuUxEyJSMeGSU3GDYvJ$ViGPA2GA
```

Note that the output is different, because the username is added to the hash.
Any change to the username will result in a different password, so any username
will have its own password.

## Usage

```
$ nobrain --help
nobrain 0.1.0

USAGE:
    nobrain [FLAGS] [OPTIONS] <domain>

FLAGS:
    -c, --confirm       Ask for password confirmation
    -h, --help          Prints help information
    -n, --no-newline    Print only the password without linebreak
    -V, --version       Prints version information

OPTIONS:
    -u, --user <username>    Add a username [default: ]

ARGS:
    <domain>
```
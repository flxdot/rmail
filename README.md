# rmail

The purpose of this CLI is to generate randomized email addresses in the form of 
`{service}-{random chars}@your-domain.com`.

This can be helpful to make your usernames less predictable or to track which service
sells you email address.

## Usage

Before you can start using `rmail`, you'll need to configure the domain of your email by
running

```bash
rmail --init
```

Afterwards you can call the tool with the name of the service the email address shall be used for

```
rmail github
```

will yield `github-34j1aq@your-domain.com` or similar.


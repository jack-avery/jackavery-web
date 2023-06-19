# jackavery-web

personal website for me.

## host
jackavery.ca is hosted on a [raspberry pi 400](https://www.raspberrypi.com/products/raspberry-pi-400/) behind [cloudflare](https://cloudflare.com).

as such, **all build directives in this repository are targeting the ARM64v8 architecture**.

## front-end
raw HTML, CSS, JS.
nginx container.
see `roles/front/files`.

## back-end
[rocket.rs](https://rocket.rs/) binary.
debian container.
see `roles/back/files`.
configured in `all.secret.yml`, see `roles/deploy/config.yml.j2` for sample.

### pre-commit
there is a pre-commit hook that you should enable to ensure you don't commit any unencrypted secret:
```
ln .hooks/pre-commit .git/hooks/pre-commit
```

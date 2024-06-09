# jackavery-web

personal website for me.

## front-end
raw HTML, CSS, JS. nginx container. see `roles/deploy/files/web`.

## back-end
[rocket.rs](https://rocket.rs/) binary. debian container. see `roles/back/files`.

configured in `all.secret.yml` & `all.yml`, see `roles/deploy/config.yml.j2` for sample.

### pre-commit
there is a pre-commit hook that you should enable to ensure you don't commit any unencrypted secret:
```
ln .hooks/pre-commit .git/hooks/pre-commit
```

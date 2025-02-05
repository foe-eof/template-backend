This crate provides opinionated config pattern used for the workspace.

We are using `config` library for configurations.

The following sources are loaded (later means higher precedence):

- config file
- env vars

All the configuration files are located in `config` directory of the project root.

It has such a hierarchy:
- `common` - used for all the profiles
- `<profile>` - any profile (if not specified via `APP_PROFILE`, set to `debug`)

You can name your profile in any way. Just don't forget to change `APP_PROFILE` env var
afterwards.

Supported file formats by the library: <https://docs.rs/config/latest/config/enum.FileFormat.html>
Only the default ones are enabled: <https://docs.rs/crate/config/latest/features>

Overridable by environment variables like `WEB_SERVER__IPV4__IP`

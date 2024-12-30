# Wilford
OAuth2 and OpenID Connect implementation.

## Authorization provders
- EspoCRM: Utilizes the EspoCRM system as it's password and user managment system
- Local: Utilizes the local database as it's password and user managment system

## Development
Requirments:
- Server
  - [Rust compiler, Cargo](https://rust-lang.org)
  - Docker
- Frontend
  - Node >= 22
  - Yarn

- Start everything with
```bash
make dev
```
This will:
- Create an OIDC signing key if it doesn't exist
- Copy `sample_config.json` to `config.json`
- Start the server and the frontend

The following services will be available:
- The backend, on port [2521](http://localhost:2512)
- The frontend, on port [3000](http://localhost:3000)

## Configuring EspoCRM
After starting, you should configure an API-client in EspoCRM:
1. Log in with EspoCRM [here](http://localhost:2524). Your username and password are `admin`
2. In the top right, select the three dots > Administration
3. Select Roles > Create Role
4. Give it a name, e.g. `admin`
5. Set `User permission` to `all` 
6. Scroll down to `Users`, set to `enabled`
7. Select `Save`
8. In Administration again, go to `API Users` > Create API User
9. Give it a name, e.g. `wilford`
10. Select the role you just created under `Roles`
11. Set `Authentication method` to `HMAC` and select `Save`
12. Copy the `API Key` and `Secret Key` to `config.json`
13. Restart Wilford
```bash
docker-compose down
make up 
```

## Generate OIDC Key
When using `make dev`, this is done automatically.

```bash
# Private key
openssl genrsa -out ./oidc.pem 4096

# Public key
openssl rsa -in ./oidc.pem -pubout -outform PEM -out ./oidc.pem.pub
```

# License
MIT or Apache-2.0, at your option

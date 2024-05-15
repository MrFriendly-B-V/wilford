# Wilford
Bolted-on OAuth2 provider using EspoCRM as credentials provider.

You create accounts in EspoCRM, 
configure permissions here in Wilford. 
Your applications will then authenticate with Wilford, 
and your users can continue using their EspoCRM login credentials.

## Development
- Start everything with
```bash
make up
```
This will:
- Create an OIDC signing key if it doesn't exist
- Copy `sample_config.json` to `config.json
- Start all containers

The following services will be available:
- The backend, on port [2521](http://localhost:2512)
- The frontend, on port [2522](http://localhost:2522)
- The docs, on port [2523](http://localhost:2523)
- EspoCRM, on port [2524](http://localhost:2524)

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


# License
MIT or Apache-2.0, at your option

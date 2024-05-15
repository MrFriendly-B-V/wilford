# Introspect
Token introspection endpoint.  
See also: [RFC7662](https://datatracker.ietf.org/doc/html/rfc7662)

>Requires [CAT](../api/index.md#cat-authorization) authorization

`POST /api/oauth/introspect`

## Body
Content-Type: `application/x-www-form-urlencoded`

```
token = <OAuth2 Access token to introspect>
scope = <(optional) space seperated list of required scopes>
```

## Response
If the token is valid and all scopes requested are present, as per [RFC7662 Section 2.2](https://datatracker.ietf.org/doc/html/rfc7662#section-2.2)
```jsonc
{
    "active": "true",
    "scope": "string",
    "client_id": "string",
    "username": "string",
    "token_type": "string",
    "exp": 0,
    "iat": 0,
    "nbf": 0,
    "sub": "string" // Espo user id
}
```
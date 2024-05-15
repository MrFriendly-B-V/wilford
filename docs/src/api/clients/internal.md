# Internal
Wilford requires its own OAuth2 client to perform operations and log the user in.
This endpoint provides information about the OAuth2 client

`GET /api/v1/clients/internal`

## Response 
```json
{
    "name": "<name of the client>",
    "client_id": "<OAuth2 client_id>",
    "redirect_uri": "<OAuth2 redirect_uri>"
}
```
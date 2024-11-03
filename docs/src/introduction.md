# Wilford

Wilford is a OAuth2 Provider using EspoCRM as its credentials provider.

## Sources
Wilford's implementation of OAuth2 and OpenID Connect is derived from the following documents:
- [RFC6749](https://datatracker.ietf.org/doc/html/rfc6749)
- [RFC6750](https://datatracker.ietf.org/doc/html/rfc6750)
- [RFC7662](https://datatracker.ietf.org/doc/html/rfc7662)
- [OpenID Connect Core 1.0](https://openid.net/specs/openid-connect-core-1_0.html)

## TODO
Not everything is implemented 100%. I'd like to add support for:
- [A.4](https://openid.net/specs/openid-connect-core-1_0.html#code-id_tokenExample) (`response_type=code id_token`).
At the moment only `response_type=id_token token` is supported.
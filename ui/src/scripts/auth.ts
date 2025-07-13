import {ApiError, ApiErrorKind} from "@/scripts/core/error";
import {Result} from "@/scripts/core/result";
import {fetch1} from "@/scripts/core/fetch1";
import {server} from "@/main";

export enum LoginStatus {
  INVALID_CREDENTIALS,
  TOTP_REQUIRED,
  EMAIL_UNVERIFIED,
  OK,
  SCOPE_ERROR
}

const EMAIL_NOT_VERIFIED_MESSAGE = "Your email address is not verified";

export class Auth {
 
  static async login(
    username: string,
    password: string,
    authorizationCode: string,
    totpCode?: string,
  ): Promise<Result<LoginStatus, ApiError>> {
    const result = await (await fetch1(`${server}/api/v1/auth/login`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        'username': username,
        'password': password,
        'totp_code': totpCode,
        'authorization': authorizationCode,
      })
    }))
      .map1(async (response) => {
        interface Payload {
          status: boolean,
          totp_required: boolean
        }
        
        const payload: Payload = await response.json();
        
        if(!payload.status && !payload.totp_required) {
          return LoginStatus.INVALID_CREDENTIALS;
        }
        
        if(!payload.status && payload.totp_required) {
          return LoginStatus.TOTP_REQUIRED;
        }
        
        return LoginStatus.OK;
      });
    
    if(result.isOk()) return result;
    
    const errorValue = result.unwrapErr();
    if(
      errorValue.kind == ApiErrorKind.Request
      && errorValue.status == 401
      && errorValue.message?.toString() === EMAIL_NOT_VERIFIED_MESSAGE) {
      return Result.ok(LoginStatus.EMAIL_UNVERIFIED);
    }
    
    if(
      errorValue.kind == ApiErrorKind.Request
      && errorValue.status == 403
    ) {
      return Result.ok(LoginStatus.SCOPE_ERROR);
    }
    
    return result;
  }
}
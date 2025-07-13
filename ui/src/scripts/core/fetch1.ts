import {Result} from "@/scripts/core/result";
import {ApiError} from "@/scripts/core/error";

export async function fetch1(input: RequestInfo | URL, init?: RequestInit): Promise<Result<Response, ApiError>> {
  if (init) {
    init.credentials = "include";
    if (init.headers) {
      init.headers = {
        ...init.headers,
        'Access-Control-Allow-Credentials': "true",
      }
    } else {
      init.headers = {
        'Access-Control-Allow-Credentials': "true",
      };
    }
  } else {
    init = {
      credentials: "include",
      headers: {
        'Access-Control-Allow-Credentials': "true",
      },
    };
  }
  
  try {
    const r = await fetch(input, init);
    if (!r.ok) {
      return Result.err(ApiError.request(r.status, await r.text()));
    }
    
    return Result.ok(r);
  } catch (error: unknown) {
    return Result.err(ApiError.request(-1, (<any> error).toString()))
  }
}
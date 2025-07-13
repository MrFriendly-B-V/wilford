import {server} from "@/main";
import {ClientInfo} from "@/scripts/clients";
import {ApiError} from "@/scripts/core/error";
import {fetch1} from "@/scripts/core/fetch1";
import {Result} from "@/scripts/core/result";

interface _User {
    name: string,
    email: string,
    espo_user_id: string,
    is_admin: boolean,
}

export class User {
    name: string;
    email: string;
    espoUserId: string;
    isAdmin: boolean;

    constructor(name: string, email: string, espoUserId: string, isAdmin: boolean) {
        this.name = name;
        this.email = email;
        this.espoUserId = espoUserId;
        this.isAdmin = isAdmin;
    }

    static async getCurrent(): Promise<UserInfo> {
        return (await (await fetch1(`${server}/api/v1/user/info`))
          .map1(async (r) => {
              if(r.status == 401) {
                  const client = await ClientInfo.getInternal();
                  window.location.href = client.getAuthorizationRedirect();
              }
              
              interface _UserInfo extends _User {
                  require_password_change: boolean,
              }
              
              const j: _UserInfo = await r.json();
              return new UserInfo(j.name, j.email, j.espo_user_id, j.is_admin, j.require_password_change);
          })
        ).unwrap()
    }

    static async list(): Promise<Result<User[], ApiError>> {
        return await (await fetch1(`${server}/api/v1/user/list`, ))
          .map1(async (response) => {
              interface Payload {
                  users: _User[]
              }
              
              const payload: Payload = await response.json();
              return payload.users.map(u => new User(u.name, u.email, u.espo_user_id, u.is_admin))
          });
    }

    async listPermittedScopes(): Promise<string[]> {
        const r = await fetch(`${server}/api/v1/user/permitted-scopes/list?user=${this.espoUserId}`, {
            headers: {
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })

        interface Response {
            scopes: string[]
        }

        const j: Response = await r.json();
        return j.scopes;
    }

    async deletePermittedScope(scope: string): Promise<Result<void, ApiError>> {
        return (await fetch1(`${server}/api/v1/user/permitted-scopes/remove`, {
            method: 'DELETE',
            body: JSON.stringify({
                from: this.espoUserId,
                scope: scope,
            }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })).mapVoid();
    }

    async addPermittedScope(scope: string): Promise<Result<void, ApiError>> {
        return (await fetch1(`${server}/api/v1/user/permitted-scopes/add`, {
            method: 'POST',
            body: JSON.stringify({
                to: this.espoUserId,
                scope: scope,
            }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })).mapVoid();
    }
    
    static async isFirstRegister(): Promise<Result<boolean, ApiError>> {
        return await (await fetch1(`${server}/api/v1/user/registration-required`))
          .map1(async (response) => {
              interface Payload {
                  registration_required: boolean,
              }
              
              const payload: Payload = await response.json();
              return payload.registration_required;
          });
    }
    
    static async passwordChangeSupported(): Promise<Result<boolean, ApiError>> {
        return await (await fetch1(`${server}/api/v1/user/supports-password-change`))
          .map1(async (response) => {
              interface Response {
                  password_change_supported: boolean;
              }
              
              const payload: Response = await response.json();
              return payload.password_change_supported;
          })
    }
    
    async updatePassword(oldPassword: string, newPassword: string): Promise<Result<void, ApiError>> {
        return (await fetch1(`${server}/api/v1/user/change-password`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                old_password: oldPassword,
                new_password: newPassword
            }),
        })).mapVoid();
    }
    
    static async register(name: string, email: string, password: string, locale: string,): Promise<Result<void, ApiError>> {
        return (await fetch1(`${server}/api/v1/user/register`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name: name,
                email: email,
                password: password,
                locale: locale,
            })
        })).mapVoid()
    }
    
    static async resetPassword(email: string): Promise<Result<void, ApiError>> {
        return (await fetch1(`${server}/api/v1/user/password-forgotten`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                email: email,
            })
        })).mapVoid();
    }
    
    static async verifyEmail(userId: string, code: string): Promise<Result<void, ApiError>> {
        return (await fetch1(`${server}/api/v1/user/verify-email?user_id=${userId}&verification_code=${code}`, {
            method: 'POST',
        })).mapVoid();
    }
    
    async changeName(newName: string): Promise<Result<void, ApiError>> {
        return (await fetch1(`${server}/api/v1/user/change-name`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                new_name: newName
            })
        })).mapVoid();
    }
    
    async changeEmail(newEmail: string, password: string): Promise<Result<void, ApiError>> {
        return (await fetch1(`${server}/api/v1/user/change-email`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                new_email: newEmail,
                password: password,
            })
        })).mapVoid();
    }
}

export class UserInfo extends User {
    requirePasswordChange: boolean;
    
    constructor(name: string, email: string, espoUserId: string, isAdmin: boolean, requirePasswordChange: boolean) {
        super(name, email, espoUserId, isAdmin);
        this.requirePasswordChange = requirePasswordChange;
    }
}
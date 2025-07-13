import {server} from "@/main";
import {ClientInfo} from "@/scripts/clients";

export class CatToken {
    name: string;
    token: string;
    
    constructor(name: string, token: string) {
        this.name = name;
        this.token = token;
    }
    
    static async list(): Promise<CatToken[]> {
        const r = await fetch(`${server}/api/v1/cat/list`, {
            headers: {
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })

        if(r.status == 401) {
            const client = await ClientInfo.getInternal();
            window.location.href = client.getAuthorizationRedirect();
        }

        interface Response {
            tokens: {
                name: string,
                token: string,
            }[]
        }

        const j: Response = await r.json();
        return j.tokens.map(t => new CatToken(t.name, t.token));
    }

    static async create(name: string) {
        const r = await fetch(`${server}/api/v1/cat/add`, {
            method: 'POST',
            body: JSON.stringify({
                name: name
            }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })

    }

    async revoke() {
        await fetch(`${server}/api/v1/cat/remove`, {
            method: 'DELETE',
            body: JSON.stringify({
                token: this.token,
            }),
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })
    }
}
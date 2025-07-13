import {server} from "@/main";

interface _ClientInfo {
    name: string,
    client_id: string,
    client_secret: string,
    redirect_uri: string,
}

export class ClientInfo {
    name: string;
    clientId: string;
    clientSecret: string;
    redirectUri: string;

    constructor(name: string, client_id: string, client_secret: string, redirect_uri: string) {
        this.name = name;
        this.clientId = client_id;
        this.clientSecret = client_secret;
        this.redirectUri = redirect_uri;
    }

    static async getInternal() : Promise<ClientInfo> {
        const r = await fetch(`${server}/api/v1/clients/internal`);
        const j: _ClientInfo = await r.json();
        return new ClientInfo(j.name, j.client_id, j.client_secret, j.redirect_uri);
    }

    getAuthorizationRedirect(manageScopes: boolean = false): string {
        const scopesParam = manageScopes ? `&scope=wilford.manage` : "";
        return `${server}/api/oauth/authorize?client_id=${this.clientId}&response_type=token${scopesParam}&redirect_uri=${this.redirectUri}`
    }

    static async new(name: string, redirectUri: string) {
        await fetch(`${server}/api/v1/clients/add`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            },
            body: JSON.stringify({
                name: name,
                redirect_uri: redirectUri,
            })
        })
    }

    static async list(): Promise<ClientInfo[]> {
        const r = await fetch(`${server}/api/v1/clients/list`, {
            headers: {
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            }
        })

        interface Response {
            clients: _ClientInfo[]
        }

        const j: Response = await r.json();
        return j.clients.map(c => new ClientInfo(c.name, c.client_id, c.client_secret, c.redirect_uri));
    }

    async remove() {
        await fetch(`${server}/api/v1/clients/remove`, {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${window.localStorage.getItem('access_token')}`
            },
            body: JSON.stringify({
                client_id: this.clientId,
            })
        })
    }
}
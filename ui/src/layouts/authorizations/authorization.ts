import {Token} from "@/scripts/token";

export async function isAuthorized(manager: boolean): Promise<boolean> {
  const token = await Token.getCurrentInfo(manager);
  if(manager) {
    return token.scopes.includes("wilford.manage");
  } else {
    return true;
  }
}
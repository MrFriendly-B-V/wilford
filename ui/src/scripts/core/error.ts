export class ApiError {
  public readonly kind: ApiErrorKind;
  public readonly status?: number;
  public readonly message?: string;
  
  private constructor(kind: ApiErrorKind, status?: number, message?: string) {
    this.kind = kind;
    this.status = status;
    this.message = message;
  }
  
  static request(status: number, message: string): ApiError {
    return new ApiError(ApiErrorKind.Request, status, message);
  }
  
}

export enum ApiErrorKind {
  Request,
}
export class Result<T, E> {
  private readonly state: ResultState;
  private readonly ok?: T;
  private readonly err?: E;
  
  private constructor(state: ResultState, ok?: T, err?: E) {
    this.state = state;
    this.ok = ok;
    this.err = err;
  }
  
  public static ok<T, E>(ok: T): Result<T, E> {
    return new Result<T, E>(ResultState.OK, ok, undefined);
  }
  
  public static err<T, E>(err: E): Result<T, E> {
    return new Result<T, E>(ResultState.ERROR, undefined, err);
  }
  
  public isOk(): boolean {
    return this.state == ResultState.OK;
  }
  
  public isErr(): boolean {
    return this.state == ResultState.ERROR;
  }
  
  public unwrap(): T {
    if (this.isOk()) {
      return this.ok!;
    }
    
    console.error("Tried to unwrap Result while it was an error");
    return undefined!;
  }
  
  public unwrapErr(): E {
    if (this.isErr()) {
      return this.err!;
    }
    
    console.error("Tried to unwrap error Result while it was ok");
    return undefined!;
  }
  
  public map<T1>(f: (t: T) => T1): Result<T1, E> {
    if (this.isOk()) {
      return Result.ok(f(this.unwrap()));
    } else {
      return Result.err(this.unwrapErr());
    }
  }
  
  public async map1<T1>(f: (t: T) => Promise<T1>): Promise<Result<T1, E>> {
    if (this.isOk()) {
      return Result.ok(await f(this.unwrap()));
    } else {
      return Result.err(this.unwrapErr());
    }
  }
}

export enum ResultState {
  OK,
  ERROR
}
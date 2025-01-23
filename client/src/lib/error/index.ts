export class UninitializedStrongholdError extends Error {
    constructor() {
        super(
            "Stronghold uninitialized. Try calling the 'init' function and supplying a password before continuing."
        );
        this.name = 'UninitializedStrongholdError';
        Object.setPrototypeOf(this, UninitializedStrongholdError.prototype);
    }
}

export class InvalidStrongholdKeyError extends Error {
    constructor(key: string) {
        super(`Invalid stronghold key: ${key}`);
        this.name = 'InvalidStrongholdKeyError';
        Object.setPrototypeOf(this, InvalidStrongholdKeyError.prototype);
    }
}

export class AlreadyInitializedStrongholdError extends Error {
    constructor() {
        super(`Stronghold already initialized. Was this intentional?`);
        this.name = 'AlreadyInitializedStrongholdError';
        Object.setPrototypeOf(
            this,
            AlreadyInitializedStrongholdError.prototype
        );
    }
}

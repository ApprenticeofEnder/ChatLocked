import { appDataDir } from '@tauri-apps/api/path';
import {
    Client,
    type Duration,
    Store,
    Stronghold,
} from '@tauri-apps/plugin-stronghold';

import {
    AlreadyInitializedStrongholdError,
    InvalidStrongholdKeyError,
    UninitializedStrongholdError,
} from '$lib/error';

export interface StrongholdContext {
    stronghold: StrongholdStore;
}

export class StrongholdStore {
    vaultPath: string = '';

    stronghold: Stronghold | null = null;
    client: Client | null = null;
    store: Store | null = null;

    async init(password: string) {
        if (this.isInitialized()) {
            throw new AlreadyInitializedStrongholdError();
        }

        console.info('Initializing stronghold.');

        this.vaultPath = `${await appDataDir()}/chatlocked.hold`;

        console.info('Loading vault.');
        const stronghold = await Stronghold.load(this.vaultPath, password);
        console.info('Loading client.');
        let client: Client;
        const clientName = 'ChatLocked Client';
        try {
            client = await stronghold.loadClient(clientName);
        } catch {
            client = await stronghold.createClient(clientName);
        }

        this.stronghold = stronghold;
        this.client = client;
        this.store = client.getStore();
        console.info('Stronghold initialized.');
    }

    isInitialized() {
        const strongholdInitialized = !!this.stronghold;
        const clientInitialized = !!this.client;
        const storeInitialized = !!this.store;

        return strongholdInitialized && clientInitialized && storeInitialized;
    }

    async setup(email: string, secretKey: string, encryptionKey: string) {
        await this.insertRecord('email', email);
        await this.insertRecord('secretKey', secretKey);
        await this.setEncryptionKey(encryptionKey);
        await this.save();
    }

    async setEncryptionKey(encryptionKey: string) {
        await this.insertRecord('encryptionKey', encryptionKey, {
            secs: 3600,
            nanos: 0,
        });
    }

    async insertRecord(key: string, value: string, lifetime?: Duration) {
        if (!this.store) {
            throw new UninitializedStrongholdError();
        }
        const data = Array.from(new TextEncoder().encode(value));
        await this.store.insert(key, data, lifetime);
    }

    async getRecord(key: string): Promise<string> {
        if (!this.store) {
            throw new UninitializedStrongholdError();
        }
        const data = await this.store.get(key);
        if (!data) {
            throw new InvalidStrongholdKeyError(key);
        }
        return new TextDecoder().decode(new Uint8Array(data));
    }

    async save() {
        if (!this.stronghold) {
            throw new UninitializedStrongholdError();
        }
        await this.stronghold.save();
    }

    async deleteRecord(key: string): Promise<string> {
        if (!this.store) {
            throw new UninitializedStrongholdError();
        }
        const data = await this.store.remove(key);
        if (!data) {
            throw new InvalidStrongholdKeyError(key);
        }
        return new TextDecoder().decode(new Uint8Array(data));
    }

    async lock() {
        try {
            await this.deleteRecord('encryptionKey');
            await this.save();
        } catch {}
        this.store = null;
        this.client = null;
        this.stronghold?.unload();
        this.stronghold = null;
    }
}

import type { Client, Stronghold } from 'tauri-plugin-stronghold-api';

export interface KeyData {
    email: string | null;
    secretKey: string | null;
    encryptionKey: string | null;
}

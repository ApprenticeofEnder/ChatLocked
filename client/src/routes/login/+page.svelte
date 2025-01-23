<script lang="ts">
    import { getContext } from 'svelte';
    import { core } from '@tauri-apps/api';

    import LoginForm from '$lib/components/form/login-form/login-form.svelte';
    import { type KeyData } from '$lib/types';
    import { goto } from '$app/navigation';

    let { data } = $props();

    const keyData: KeyData = getContext('keyData');

    let email: string | null = $state(keyData.email);
    let password: string | null = $state(null);
    let secretKey: string | null = $state(keyData.secretKey);

    $effect(() => {
        if (!email || !secretKey) {
            goto('/setup');
        }

        if (!password) {
            return;
        }

        async function processNewEncryptionKey(
            email: string,
            password: string,
            secretKey: string
        ) {
            const encryptionKey = await core.invoke<string | null>(
                'create_encryption_key',
                {
                    email,
                    password,
                    secret_key: secretKey,
                }
            );
            if (!encryptionKey) {
                throw Error(
                    `Null encryption key. Email: ${email}. Secret Key: ${secretKey}.`
                );
            }
            window.localStorage.setItem('secretKey', secretKey || '');
            window.localStorage.setItem('email', email || '');

            keyData.encryptionKey = encryptionKey;
        }

        processNewEncryptionKey(email, password, secretKey);
    });
</script>

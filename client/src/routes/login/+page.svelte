<script lang="ts">
    import { core } from '@tauri-apps/api';
    import { getContext } from 'svelte';

    import { goto } from '$app/navigation';
    import LoginForm from '$lib/components/form/login-form/login-form.svelte';
    import type { StrongholdContext } from '$lib/stores/stronghold';

    const { stronghold }: StrongholdContext = getContext('stronghold');

    let { data } = $props();

    let password: string | null = $state(null);

    $effect(() => {
        if (!window.localStorage.getItem('isSetup')) {
            goto('/setup');
            return;
        }

        if (stronghold.isInitialized()) {
            goto('/');
            return;
        }

        if (!password) {
            return;
        }

        async function processLogin(password: string) {
            await stronghold.init(password);

            const email = await stronghold.getRecord('email');
            const secretKey = await stronghold.getRecord('secretKey');

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

            goto('/');
        }

        processLogin(password);
    });
</script>

<main>
    <h1>ChatLocked Login</h1>
    <LoginForm data={data.form} bind:password />
</main>

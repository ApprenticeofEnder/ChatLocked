<script lang="ts">
    import { core } from '@tauri-apps/api';
    import { getContext } from 'svelte';

    import { goto } from '$app/navigation';
    import SetupForm from '$lib/components/form/setup-form/setup-form.svelte';
    import Button from '$lib/components/ui/button/button.svelte';
    import type { StrongholdContext } from '$lib/stores/stronghold';

    let { data } = $props();
    let email: string | null = $state(null);
    let password: string | null = $state(null);
    let secretKey: string | null = $state(null);

    const { stronghold }: StrongholdContext = getContext('stronghold');

    let needsSetup = $state(true);
    let setupInProgress = $state(false);
    let currentStatus = $state('');

    let error = $state<unknown | null>(null);

    $inspect(error);

    function finishSetup() {
        window.localStorage.setItem('isSetup', '1');
        goto('/');
    }

    $effect(() => {
        if (window.localStorage.getItem('isSetup')) {
            return finishSetup();
        }

        if (!email || !password || !secretKey) {
            return;
        }

        async function processSetup(
            email: string,
            password: string,
            secretKey: string
        ) {
            needsSetup = false;

            currentStatus = 'Initializing vault...';

            await stronghold.init(password);

            currentStatus = 'Creating encryption key...';
            const encryptionKey = await core.invoke<string | null>(
                'create_encryption_key',
                {
                    email,
                    password,
                    secretKey,
                }
            );
            if (!encryptionKey) {
                throw Error(
                    `Null encryption key. Email: ${email}. Secret Key: ${secretKey}.`
                );
            }
            currentStatus = 'Finishing up . . .';

            await stronghold.setup(email, secretKey, encryptionKey);
            console.info('Setup complete.');
        }

        processSetup(email, password, secretKey).catch((err) => {
            error = err;
            console.error(err);
        });
    });
</script>

<main class="flex flex-col h-screen justify-center gap-2 items-center">
    <h1>ChatLocked Setup</h1>
    {#if needsSetup}
        <SetupForm data={data.form} bind:email bind:password bind:secretKey />
    {:else if setupInProgress}
        <p>{currentStatus}</p>
    {:else}
        <p>Email: {email}</p>
        <p>Password: {'*'.repeat(password?.length || 0)}</p>
        <p>Secret Key: {secretKey}</p>
        <Button on:click={finishSetup}>Complete Setup</Button>
    {/if}
</main>

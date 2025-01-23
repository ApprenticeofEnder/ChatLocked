<script lang="ts">
    import { getContext } from 'svelte';

    import { goto } from '$app/navigation';
    import type { StrongholdContext } from '$lib/stores/stronghold';

    const { stronghold }: StrongholdContext = getContext('stronghold');

    $inspect(getContext('stronghold'));

    $effect(() => {
        if (stronghold.isInitialized()) {
            return;
        }

        const isSetup = window.localStorage.getItem('isSetup');
        if (!isSetup) {
            goto('/setup');
            return;
        }

        goto('/login');
    });
</script>

<main>
    <h1>ChatLocked</h1>
</main>

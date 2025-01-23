<script lang="ts">
    import { setContext } from 'svelte';

    import {
        type StrongholdContext,
        StrongholdStore,
    } from '$lib/stores/stronghold';
    import { type KeyData } from '$lib/types';

    import '../app.css';

    const strongholdContext: StrongholdContext = $state({
        stronghold: new StrongholdStore(),
    });

    let keyData: KeyData = $state({
        email: null,
        secretKey: null,
        encryptionKey: null,
    });
    setContext('keyData', keyData);
    setContext('stronghold', strongholdContext);
    let { children } = $props();
</script>

{@render children?.()}

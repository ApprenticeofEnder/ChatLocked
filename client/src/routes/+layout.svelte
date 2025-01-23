<script lang="ts">
    import { setContext } from 'svelte';

    import { type KeyData } from '$lib/types';
    import '../app.css';

    let keyData: KeyData = $state({ email: null, secretKey: null, encryptionKey: null });
    setContext('keyData', keyData);
    let { children } = $props();
</script>

{@render children?.()}

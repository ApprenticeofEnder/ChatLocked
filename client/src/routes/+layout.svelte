<script lang="ts">
    import '../app.css';
    import { setContext } from 'svelte';

    let keyData = $state({ email: null, secretKey: null, encryptionKey: null });
    setContext('keyData', keyData);
    let { children } = $props();
</script>

{@render children?.()}

<script lang="ts">
    import { goto } from '$app/navigation';

    // Load email and secret key from local storage, if they don't exist, go to setup

    goto('/setup');
</script>

<main>
    <h1>ChatLocked</h1>
</main>

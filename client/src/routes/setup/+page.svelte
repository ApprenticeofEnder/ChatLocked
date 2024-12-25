<script lang="ts">
    import SetupForm from '$lib/components/form/setup-form/setup-form.svelte';

    let { data } = $props();
    let email: string | null = $state(null);
    let password: string | null = $state(null);
    let secretKey: string | null = $state(null);

    $effect(() => {
        if(!email || !password || !secretKey){
            return;
        }
        console.log(email, password, secretKey);
        window.localStorage.setItem("secretKey", secretKey);
        window.localStorage.setItem("email", email);
        // Store the secret key in localStorage
        // Create an actual encryption key
        // Or maybe just an encryption key for the actual key?
    })

</script>

<main>
    <h1>ChatLocked Setup</h1>
    <SetupForm data={data.form} bind:email bind:password bind:secretKey />
    <p>Email: {email}</p>
    <p>Password: {'*'.repeat(password?.length || 0)}</p>
    <p>Secret Key: {secretKey}</p>

</main>

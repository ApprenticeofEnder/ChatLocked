<script lang="ts">
    import * as Form from '$lib/components/ui/form';
    import { Input } from '$lib/components/ui/input';
    import { setupFormSchema, type SetupFormSchema } from './schema';
    import {
        type SuperValidated,
        type Infer,
        superForm,
    } from 'sveltekit-superforms';
    import { zodClient } from 'sveltekit-superforms/adapters';
    import { core } from '@tauri-apps/api';

    let {
        data,
        email = $bindable(),
        password = $bindable(),
        secretKey = $bindable(),
    }: {
        data: SuperValidated<Infer<SetupFormSchema>>;
        email: FormDataEntryValue | null;
        password: FormDataEntryValue | null;
        secretKey: string | null;
    } = $props();

    const form = superForm(data, {
        validators: zodClient(setupFormSchema),
        async onSubmit({ formData, cancel }) {
            const result = await form.validateForm();
            form.errors.update((currentErrors) => {
                return { ...currentErrors, ...result.errors };
            });
            if (!result.valid) {
                cancel();
                return;
            }
            email = formData.get('email');
            password = formData.get('password');
            secretKey = await core.invoke('generate_secret_key');
            cancel();
        },
    });

    const { form: formData, errors, enhance } = form;
</script>

<form method="POST" use:enhance>
    <Form.Field {form} name="email">
        <Form.Control let:attrs>
            <Form.Label>Email</Form.Label>
            <Input {...attrs} bind:value={$formData.email} />
        </Form.Control>
        <Form.Description>Your email.</Form.Description>
        <Form.FieldErrors />
    </Form.Field>

    <Form.Field {form} name="password">
        <Form.Control let:attrs>
            <Form.Label>Password</Form.Label>
            <Input type="password" {...attrs} bind:value={$formData.password} />
        </Form.Control>
        <Form.Description>Your password.</Form.Description>
        <Form.FieldErrors />
    </Form.Field>
    <Form.Button>Submit</Form.Button>
</form>

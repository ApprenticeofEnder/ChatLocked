<script lang="ts">
    import * as Form from '$lib/components/ui/form';
    import { Input } from '$lib/components/ui/input';
    import { loginFormSchema, type LoginFormSchema } from './schema';
    import {
        type SuperValidated,
        type Infer,
        superForm,
    } from 'sveltekit-superforms';
    import { zodClient } from 'sveltekit-superforms/adapters';

    let {
        data,
        password = $bindable(),
    }: {
        data: SuperValidated<Infer<LoginFormSchema>>;
        password: FormDataEntryValue | null;
    } = $props();

    const form = superForm(data, {
        validators: zodClient(loginFormSchema),
        async onSubmit({ formData, cancel }) {
            const result = await form.validateForm();
            form.errors.update((currentErrors) => {
                return { ...currentErrors, ...result.errors };
            });
            if (!result.valid) {
                cancel();
                return;
            }
            password = formData.get('password');
            cancel();
        },
    });

    const { form: formData, enhance } = form;
</script>

<form method="POST" use:enhance>
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

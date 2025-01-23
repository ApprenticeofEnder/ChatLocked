import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';

import { superValidate } from 'sveltekit-superforms';
import { loginFormSchema } from '$lib/components/form/login-form/schema';
import { zod } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async () => {
    return {
        form: await superValidate(zod(loginFormSchema)),
    };
};

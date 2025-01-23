import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';

import { loginFormSchema } from '$lib/components/form/login-form/schema';

import type { Actions, PageServerLoad } from './$types.js';

export const load: PageServerLoad = async () => {
    return {
        form: await superValidate(zod(loginFormSchema)),
    };
};

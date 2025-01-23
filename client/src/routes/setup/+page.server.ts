import { fail } from '@sveltejs/kit';
import { superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';

import { setupFormSchema } from '$lib/components/form/setup-form/schema';

import type { Actions, PageServerLoad } from './$types.js';

export const load: PageServerLoad = async () => {
    return {
        form: await superValidate(zod(setupFormSchema)),
    };
};

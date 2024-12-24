import type { PageServerLoad, Actions } from './$types.js';
import { fail } from '@sveltejs/kit';

import { superValidate } from 'sveltekit-superforms';
import { setupFormSchema } from '$lib/components/form/setup-form/schema';
import { zod } from 'sveltekit-superforms/adapters';

export const load: PageServerLoad = async () => {
    return {
        form: await superValidate(zod(setupFormSchema)),
    };
};

// export const actions: Actions = {
//     default: async (event) => {
//         const form = await superValidate(event, zod(setupFormSchema));
//         if (!form.valid) {
//             return fail(400, {
//                 form,
//             });
//         }
//         return {
//             form,
//         };
//     },
// };

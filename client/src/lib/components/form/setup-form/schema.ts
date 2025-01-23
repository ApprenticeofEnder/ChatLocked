import { z } from 'zod';

export const setupFormSchema = z.object({
    email: z.string().email(),
    password: z.string().min(8),
});

export type SetupFormSchema = typeof setupFormSchema;

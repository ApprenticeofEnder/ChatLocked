import { z } from 'zod';

export const loginFormSchema = z.object({
    password: z.string(),
});

export type LoginFormSchema = typeof loginFormSchema;

import { z } from "zod"

export const addressSchema = z.object({
  street: z
    .string()
    .min(5, "La calle debe tener al menos 5 caracteres")
    .max(100, "La calle no puede tener más de 100 caracteres"),

  intNumber: z
    .string()
    .max(10, "El número interior no puede tener más de 10 caracteres"),

  postalCode: z
    .string()
    .length(5, "El código postal debe ser de 5 dígitos")
    .regex(/^\d+$/, "El código postal solo debe contener números"),

  state: z
    .string()
    .min(3, "El estado debe tener al menos 3 caracteres")
    .max(50, "El estado no puede tener más de 50 caracteres"),

  city: z
    .string()
    .min(3, "La ciudad debe tener al menos 3 caracteres")
    .max(50, "La ciudad no puede tener más de 50 caracteres"),

  colony: z
    .string()
    .min(3, "La colonia debe tener al menos 3 caracteres")
    .max(50, "La colonia no puede tener más de 50 caracteres"),
})

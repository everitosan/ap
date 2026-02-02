import { z } from "zod";

export const telephoneSchema = z
  .string()
  .min(10, "El teléfono debe ser de 10 dígitos")
  .max(10, "El teléfono debe ser de 10 dígitos")
  .regex(/^\d+$/, "El teléfono solo debe contener números");
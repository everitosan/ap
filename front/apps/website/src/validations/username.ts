import { z } from "zod";

export const usernameSchema = z
  .string()
  .min(3, "El pseudónimo debe ser de al menos 3 carácteres")
  .max(10, "El pseudónimo no puede tener más de 10 carácteres");
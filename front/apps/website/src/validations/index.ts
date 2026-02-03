import { z } from "zod"
import {telephoneSchema} from "./telephone"
import {usernameSchema} from "./username"

type validationSchema = "telephone" | "username"

const schemas = new Map<validationSchema, z.ZodString>()
schemas.set("telephone", telephoneSchema)
schemas.set("username", usernameSchema)

export const validate = (schema: validationSchema, data: string | number) => new Promise((resolve, reject) => {
  const sch = schemas.get(schema)
  if (sch) {
    const result = sch.safeParse(data)
    if (!result.success) {
      reject(result.error.issues[0].message)
    } else {
      resolve(result.data)
    }
  } else {
    reject("invalid schema")
  }
})
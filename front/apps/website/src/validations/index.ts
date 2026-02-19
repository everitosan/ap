import { z } from "zod"
import {telephoneSchema} from "./telephone"
import {usernameSchema} from "./username"
import {addressSchema} from "./address"

type validationSchema = "telephone" | "username" | "address"

const schemas = new Map<validationSchema, z.ZodString | z.ZodObject<any>>()
schemas.set("telephone", telephoneSchema)
schemas.set("username", usernameSchema)
schemas.set("address", addressSchema)

export const validate = (schema: validationSchema, data: string | number | object) => new Promise((resolve, reject) => {
  const sch = schemas.get(schema)
  if (sch) {
    const result = sch.safeParse(data)
    if (!result.success) {
      // For object schemas, return all field errors
      if (typeof data === "object" && !Array.isArray(data)) {
        const fieldErrors: Record<string, string> = {}
        result.error.issues.forEach(issue => {
          const field = issue.path[0]
          if (field && typeof field === "string" && !fieldErrors[field]) {
            fieldErrors[field] = issue.message
          }
        })
        reject(fieldErrors)
      } else {
        reject(result.error.issues[0].message)
      }
    } else {
      resolve(result.data)
    }
  } else {
    reject("invalid schema")
  }
})
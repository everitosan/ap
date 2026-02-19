import { api } from "@/api/ap"

export const loginOrRegister = (telephone: string) => {
  return api.login(telephone)
}

export const validateCode = (code: string) => {
  return api.validateCode(code)
}

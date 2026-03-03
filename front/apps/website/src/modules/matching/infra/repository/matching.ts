import { api } from "@/api/ap"

export const requestMatch = () => {
  return api.requestMatch()
}

export const getMatchStatus = () => {
  return api.getMatchStatus()
}

export const cancelMatch = () => {
  return api.cancelMatch()
}

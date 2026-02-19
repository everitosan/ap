import { api } from "@/api/ap"

export const fillProfile = (username: string, topics: string[]) => {
  return api.fillProfile(username, topics)
}

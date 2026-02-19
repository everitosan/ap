import { api, type AddressData } from "@/api/ap"

export const fillProfile = (username: string, topics: string[]) => {
  return api.fillProfile(username, topics)
}

export const fillAddress = (address: AddressData) => {
  return api.fillAddress(address)
}

import { Fetch } from "justfetch-ts"
import type { Topic } from "@/modules/catalogues/domain/topic"

export type ApiResponse<T> = {
  data: T
}

export type ApiError = {
  error: string
  message: string
}

export type UserResponse = {
  username: string | null
  topics: unknown | null
  created: string
  filled_address: boolean
}

export type AddressData = {
  street: string
  int_number: string
  postal_code: string
  state: string
  city: string
  colony: string
}

export type PartnerData = {
  username: string | null
  address: AddressData | null
}

export type MatchStatusResponse = {
  status: "paired" | "queued" | "already_paired" | "already_queued" | "idle"
  partner: PartnerData | null
  queued_at: string | null
}

class ApApi extends Fetch {
  login(telephone: string): Promise<ApiResponse<string>> {
    return this.post("/api/v1/login", { payload: { telephone } })
  }

  register(telephone: string): Promise<ApiResponse<string>> {
    return this.post("/api/v1/register", { payload: { telephone } })
  }

  validateCode(code: string): Promise<ApiResponse<string>> {
    return this.post("/api/v1/phone-validate", { payload: { code } })
  }

  resendCode(): Promise<ApiResponse<string>> {
    return this.post("/api/v1/resend-code", {})
  }

  getUser(): Promise<ApiResponse<UserResponse>> {
    return this.get("/api/v1/user")
  }

  getTopics(): Promise<ApiResponse<Topic[]>> {
    return this.get("/api/v1/topics")
  }

  fillProfile(username: string, topics: string[]): Promise<ApiResponse<string>> {
    return this.post("/api/v1/fill-profile", { payload: { username, topics } })
  }

  fillAddress(address: AddressData): Promise<ApiResponse<string>> {
    return this.post("/api/v1/fill-address", { payload: address })
  }

  requestMatch(): Promise<ApiResponse<MatchStatusResponse>> {
    return this.post("/api/v1/match/request", {})
  }

  getMatchStatus(): Promise<ApiResponse<MatchStatusResponse>> {
    return this.get("/api/v1/match/status")
  }

  cancelMatch(): Promise<ApiResponse<string>> {
    return this.delete("/api/v1/match")
  }
}

export const api = new ApApi("http://localhost:9002")
  .addHeader("Content-Type", "application/json")
  .setOptions({ credentials: "include" })

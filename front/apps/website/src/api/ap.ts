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
}

export const api = new ApApi("http://localhost:9002")
  .addHeader("Content-Type", "application/json")
  .setOptions({ credentials: "include" })

import { Fetch } from "justfetch-ts"

export type ApiResponse<T> = {
  data: T
}

export type ApiError = {
  error: string
  message: string
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
}

export const api = new ApApi("http://localhost:9002")
  .addHeader("Content-Type", "application/json")
  .setOptions({ credentials: "include" })

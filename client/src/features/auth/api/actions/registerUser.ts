import { apiAxios } from "@/lib/api-axios"

export const registerUser = () => {
  apiAxios.post("login")
}
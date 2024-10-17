import axios from "@/api";

export const fetchSearch = (params?: any) => {
  return axios.get("/api/search", { params });
};

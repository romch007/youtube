import axios from "axios";

const api = axios.create({
  baseURL: "http://localhost:8080",
  timeout: 5000,
});

api.interceptors.request.use((config) => {
  const token = localStorage.getItem("token");
  if (token) {
    config.headers["Authorization"] = `Bearer ${token}`;
  }
  return config;
});

export interface Video {
  id: number;
  title: string;
  description: string;
  bucket: string;
  author_id: string;
  author: User;
  published_at: string;
}

export interface User {
  id: number;
  username: string;
}

export async function fetcher<T>(url: string): Promise<T> {
  const response = await api.get<T>(url);
  return response.data;
}

export function resolveVideo(bucket: string): string {
  return `http://localhost:9000/videos/${bucket}`;
}

export default api;

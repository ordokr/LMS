export interface CanvasConfig {
  base_url: string;
  api_token: string;
  timeout_seconds: number | null;
}

export interface DiscourseConfig {
  base_url: string;
  api_key: string;
  api_username: string;
  timeout_seconds: number | null;
}

export interface ApiConfig {
  canvas: CanvasConfig;
  discourse: DiscourseConfig;
}

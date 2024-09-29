declare global {
  namespace NodeJS {
    interface ProcessEnv {
      BACKEND_PORT?: string;
      HOST?: string;
      PROTOCOL?: string;
    }
  }
}
export {}

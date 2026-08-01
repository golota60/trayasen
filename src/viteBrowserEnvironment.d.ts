export declare const resolveViteNodeEnvironment: (
  command: string,
  nodeEnvironment: string | undefined
) => "development" | "production";

export declare const getBrowserProcessEnvironment: (
  nodeEnvironment: string
) => string;

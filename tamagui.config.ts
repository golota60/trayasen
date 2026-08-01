import { config as baseConfig } from "@tamagui/config/v3";
import { createTamagui } from "tamagui";

export const darkTheme = {
  background: "#0B0F14",
  backgroundHover: "#121A25",
  backgroundPress: "#182231",
  backgroundFocus: "#182231",
  backgroundStrong: "#111823",
  backgroundTransparent: "rgba(11, 15, 20, 0)",
  color: "#F5F7FA",
  colorHover: "#FFFFFF",
  colorPress: "#FFFFFF",
  colorFocus: "#FFFFFF",
  colorTransparent: "rgba(245, 247, 250, 0)",
  borderColor: "#263241",
  borderColorHover: "#344458",
  borderColorPress: "#F28C28",
  borderColorFocus: "#F28C28",
  accent: "#F28C28",
  accentHover: "#FFA447",
  accentPress: "#D97716",
  accentColor: "#17100A",
  muted: "#98A5B5",
  error: "#F97066",
  errorBackground: "#321B1D",
  success: "#67D59C",
  successBackground: "#123024",
};

const appConfig = createTamagui({
  ...baseConfig,
  themes: { dark: darkTheme },
  settings: {
    ...baseConfig.settings,
    defaultFont: "body",
  },
});

export type AppConfig = typeof appConfig;

declare module "tamagui" {
  interface TamaguiCustomConfig extends AppConfig {}
}

export default appConfig;

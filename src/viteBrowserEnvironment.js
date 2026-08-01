export const resolveViteNodeEnvironment = (command, nodeEnvironment) => {
  if (nodeEnvironment === "development" || nodeEnvironment === "production") {
    return nodeEnvironment;
  }

  return command === "build" ? "production" : "development";
};

export const getBrowserProcessEnvironment = (nodeEnvironment) =>
  JSON.stringify({
    NODE_ENV: nodeEnvironment,
    TAMAGUI_TARGET: "web",
  });

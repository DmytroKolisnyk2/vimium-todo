export const customRouter = (innerRouter, extraRoutes = []) => {
  let routes;
  return {
    get prefix() {
      return innerRouter.prefix;
    },

    get routes() {
      if (!routes) routes = innerRouter.routes.concat(extraRoutes);
      return [...extraRoutes, ...innerRouter.routes];
    },
  };
};

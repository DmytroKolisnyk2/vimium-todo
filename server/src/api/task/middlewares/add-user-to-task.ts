export default (_, { strapi }) => {
  return async (ctx, next) => {
    try {
      const { user } = ctx.state;

      if (ctx.request?.body?.data) ctx.request.body.data.user = user.id;

      await next();
    } catch (error) {
      console.log(error);
    }
  };
};

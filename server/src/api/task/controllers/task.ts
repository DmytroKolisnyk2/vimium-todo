/**
 * task controller
 */

import { factories } from "@strapi/strapi";

export default factories.createCoreController(
  "api::task.task",
  ({ strapi }) => ({
    getMyTasks: async (ctx) => {
      const { user } = ctx.state;

      const tasks = await await strapi.db
        .query("api::task.task")
        .findMany({ where: { user: user.id } });

      return tasks;
    },

    getCompletedTasks: async (ctx) => {
      const { user } = ctx.state;

      const tasks = await await strapi.db
        .query("api::task.task")
        .findMany({ where: { user: user.id, isCompleted: true } });
      
      return tasks;
    },
  })
);

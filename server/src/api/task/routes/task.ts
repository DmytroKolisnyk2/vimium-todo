/**
 * task router
 */

import { factories } from "@strapi/strapi";
import { customRouter } from "../../../../utils/custom-router";

const extraRoutes = [
  {
    method: "GET",
    path: "/tasks/my",
    handler: "task.getMyTasks",
    config: {
      policies: ["global::is-authenticated"],
    },
  },
  {
    method: "GET",
    path: "/tasks/completed",
    handler: "task.getCompletedTasks",
    config: {
      policies: ["global::is-authenticated"],
    },
  },
];

const defaultRouter = factories.createCoreRouter("api::task.task", {
  config: {
    update: {
      policies: ["global::is-authenticated", "api::task.is-own-task"],
    },
    create: {
      policies: ["global::is-authenticated"],
      middlewares: ["api::task.add-user-to-task"],
    },
    delete: {
      policies: ["global::is-authenticated", "api::task.is-own-task"],
    },
  },
});

export default customRouter(defaultRouter, extraRoutes);

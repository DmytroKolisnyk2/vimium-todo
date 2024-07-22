export default async (policyContext, _, { strapi }) => {
  try {
    const userId = policyContext?.state?.user?.id;
    const taskId = policyContext?.params?.id;

    const task = await strapi.db
      .query("api::task.task")
      .findOne({ where: { id: taskId }, populate: ["user"] });

    if (task?.user?.id === userId) {
      console.log("is-own-task policy: user is the owner of the task");
      return true;
    }

    return false;
  } catch (error) {
    console.log("is-own-task policy error", error);
    return false;
  }
};

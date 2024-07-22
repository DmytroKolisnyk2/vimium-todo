export default (policyContext) => {
  if (policyContext.state.user) {
    return true;
  }

  return false;
};

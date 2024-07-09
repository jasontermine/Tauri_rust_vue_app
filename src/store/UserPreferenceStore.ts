import { defineStore } from "pinia";

export const useUserPreferenceStore = defineStore("UserPreferenceStore", {
  state: () => ({
    path: "",
  }),

  actions: {
    setPath(path: string) {
      this.path = path;
    },
  },

  getters: {
    getPath(state) {
      return state.path;
    },
  },
});

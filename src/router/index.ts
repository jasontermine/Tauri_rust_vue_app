import { createRouter, createWebHistory } from 'vue-router';
import { routes } from '@/router/routes';

const router = createRouter({
  history: createWebHistory(),
  routes: routes,
  scrollBehavior(to, _, savedPosition) {
    if (savedPosition) {
      console.log("savedPosition")
      return savedPosition;
    } else if (to.hash && to.hash.substring(0, 7) !== '#state=') {
      // don't try to scroll when keycloak redirects to this page
      return {
        el: to.hash,
        //offset: { left: 0, top: NAVBAR_HEIGHT_OFFSET },
      };
    } else {
      return { left: 0, top: 0 };
    }
  },
});

export default router;
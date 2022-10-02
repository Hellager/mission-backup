import { createRouter, createWebHistory, RouteLocationNormalized } from 'vue-router';
import TablePage from '../views/TablePage.vue';
import MissionConfig from '../views/MissionConfig.vue';
import GlobalSetting from '../views/GlobalSetting.vue';
// import LockPage from '../views/LockPage.vue';
import StatisticPage from '../views/StatisticPage.vue';

const routes = [
    {
        path: '/',
        redirect: (to: RouteLocationNormalized) => {
            return {
                path: '/table',
            }
        }
    },
    {
        path: '/table',
        component: TablePage,
    },
    {
        path: '/config',
        component: MissionConfig,
        props: (route: any) => ({ mode: route.query.mode })
    },
    {
        path: '/setting',
        component: GlobalSetting,
    },
    // {
    //     path: '/lock',
    //     component: LockPage,
    // },
    {
        path: '/statistic',
        component: StatisticPage
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes,
    scrollBehavior (to: any, from: any, savedPosition: any) {
        return {
            top: 0,
            behavior: 'smooth',
        }    
    }
})

router.afterEach((to, from) => {
    to.meta.transitionName = "";

    if (to.path == '/config') {
        to.meta.transitionName = "route-slide-in-right";
    } else if (to.path == '/lock') {
        to.meta.transitionName = "route-slide-in-left";
    } else if (to.path == '/setting') {
        to.meta.transitionName = "route-slide-in-up";
    } else if (to.path == '/statistic') {
        to.meta.transitionName = "route-slide-in-down";
    } else if (to.path == '/table' && from.path == '/config') {
        to.meta.transitionName = "route-slide-out-right";
    } else if (to.path == '/table' && from.path == '/lock') {
        to.meta.transitionName = "route-slide-out-left";
    } else if (to.path == '/table' && from.path == '/setting') {
        to.meta.transitionName = "route-slide-out-up";
    } else if (to.path == '/table' && from.path == '/statistic') {
        to.meta.transitionName = "route-slide-out-down";
    }
})

export default router;
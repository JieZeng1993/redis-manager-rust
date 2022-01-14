export default [
  {
    path: '/user',
    //表示隐藏菜单、导航，footer，可以手动配置
    layout: false,
    routes: [
      {
        path: '/user',
        routes: [
          {
            name: 'login',
            path: '/user/login',
            component: './User/Login',
          },
        ],
      },
      {
        component: './404',
      },
    ],
  },
  {
    path: '/welcome',
    name: 'welcome',
    icon: 'smile',
    component: './Welcome',
  },
  {
    path: '/admin',
    name: 'admin',
    icon: 'crown',
    access: 'canAdmin',
    component: './Admin',
    routes: [
      {
        path: '/admin/sub-page',
        name: 'sub-page',
        icon: 'smile',
        component: './Welcome',
      },
      {
        component: './404',
      },
    ],
  },
  {
    path: '/redisManage',
    name: 'redisManage',
    icon: 'crown',
    //权限控制
    // access: 'canAdmin',
    routes: [
      {
        path: '/redisManage/redisInfo',
        name: 'redisInfo',
        icon: 'smile',
        component: './RedisManage/RedisInfo',
      },
      {
        path: '/redisManage/redisInfo/update/:id',
        // name: 'redisInfoUpdate',
        icon: 'smile',
        component: './RedisManage/RedisInfo/update',
      },
      {
        path: '/redisManage/redisInfo/insert',
        // name: 'redisInfoUpdate',
        icon: 'smile',
        component: './RedisManage/RedisInfo/update',
      },
    ],
  },
  {
    name: 'list.table-list',
    icon: 'table',
    path: '/list',
    component: './TableList',
  },
  {
    path: '/',
    redirect: '/welcome',
  },
  {
    component: './404',
  },
];

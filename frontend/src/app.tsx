import type {Settings as LayoutSettings} from '@ant-design/pro-layout';
import {PageLoading, SettingDrawer} from '@ant-design/pro-layout';
import type {RunTimeLayoutConfig} from 'umi';
import {history, Link, RequestConfig} from 'umi';
import RightContent from '@/components/RightContent';
import Footer from '@/components/Footer';
import {currentUser as queryCurrentUser} from './services/ant-design-pro/userApi';
import {BookOutlined, LinkOutlined} from '@ant-design/icons';
import {RequestOptionsInit} from "umi-request";

const isDev = process.env.NODE_ENV === 'development';
const loginPath = '/user/login';

/** 获取用户信息比较慢的时候会展示一个 loading */
export const initialStateConfig = {
  loading: <PageLoading/>,
};

/**
 * @see  https://umijs.org/zh-CN/plugins/plugin-initial-state
 * */
export async function getInitialState(): Promise<{
  settings?: Partial<LayoutSettings>;
  currentUser?: USER_API.CurrentUser;
  loading?: boolean;
  fetchUserInfo?: () => Promise<USER_API.CurrentUser | undefined>;
}> {
  const fetchUserInfo = async () => {
    try {
      const msg = await queryCurrentUser();
      return msg.data;
    } catch (error) {
      history.push(loginPath);
    }
    return undefined;
  };
  // 如果是登录页面，不执行
  if (history.location.pathname !== loginPath) {
    const currentUser = await fetchUserInfo();
    return {
      fetchUserInfo,
      currentUser,
      settings: {},
    };
  }
  return {
    fetchUserInfo,
    settings: {},
  };
}

// ProLayout 支持的api https://procomponents.ant.design/components/layout
export const layout: RunTimeLayoutConfig = ({initialState, setInitialState}) => {
  return {
    rightContentRender: () => <RightContent/>,
    disableContentMargin: false,
    waterMarkProps: {
      content: initialState?.currentUser?.name,
    },
    footerRender: () => <Footer/>,
    onPageChange: () => {
      const {location} = history;
      // 如果没有登录，重定向到 login
      if (!initialState?.currentUser && location.pathname !== loginPath) {
        history.push(loginPath);
      }
    },
    links: isDev
      ? [
        <Link to="/umi/plugin/openapi" target="_blank">
          <LinkOutlined/>
          <span>OpenAPI 文档</span>
        </Link>,
        <Link to="/~docs">
          <BookOutlined/>
          <span>业务组件文档</span>
        </Link>,
      ]
      : [],
    menuHeaderRender: undefined,
    // 自定义 403 页面
    // unAccessible: <div>unAccessible</div>,
    // 增加一个 loading 的状态
    childrenRender: (children, props) => {
      // if (initialState?.loading) return <PageLoading />;
      return (
        <>
          {children}
          {!props.location?.pathname?.includes('/login') && (
            <SettingDrawer
              enableDarkTheme
              settings={initialState?.settings}
              onSettingChange={(settings) => {
                setInitialState((preInitialState) => ({
                  ...preInitialState,
                  settings,
                }));
              }}
            />
          )}
        </>
      );
    },
    menu: {
      // 每当 initialState?.currentUser?.userid 发生修改时重新执行 request
      params: {
        userId: initialState?.currentUser?.id,
      },
      request: async (params, defaultMenuData) => {
        // initialState.currentUser 中包含了所有用户信息
        // const menuData = await fetchMenuData();
        console.log(defaultMenuData)
        return defaultMenuData;
      },
    },
    ...initialState?.settings,
  };
};

export const request: RequestConfig = {
  middlewares: [
    async function middlewareA(ctx: any, next: () => any) {
      const authorization = localStorage.getItem("authorization");
      const {req} = ctx;
      const {options} = req;
      if (authorization) {
        options.headers = {
          ...options.headers,
          Authorization: authorization
        }
      }

      if (!options.headers["Content-Type"]) {
        options.headers = {
          ...options.headers,
          'Content-Type': 'application/json'
        }
      }

      await next();
    },
  ],
  responseInterceptors: [
    async function responseInterceptor(response: Response, options: RequestOptionsInit) {
      return response;
    }
  ],
  errorConfig: {
    adaptor: (resData) => {
      return {
        ...resData,
        errorMessage: resData.msg,
      };
    },
  },
}


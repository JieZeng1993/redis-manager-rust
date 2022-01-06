import {request} from 'umi';
import {Response} from "@/services/ant-design-pro/index";

/**
 * 用户相关的接口
 */

/** 登录接口 POST /api/redisInfo/page */
export async function redisInfoPage(body: REDIS_API.RedisInfoPageDto, options?: { [key: string]: any }) {
  if (body.updateTimeRange) {
    // @ts-ignore
    body.updateTimeBegin = body.updateTimeRange[0];
    // @ts-ignore
    body.updateTimeEnd = body.updateTimeRange[1];
    // @ts-ignore
    body.updateTimeRange = undefined;
  }
  return request<Response<REDIS_API.RedisInfoVo[]>>('/api/redisInfo/page', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    data: body,
    ...(options || {}),
  });
}


/** 登录接口 POST /api/redisInfo/:id */
export async function redisInfoFindById(id: number) {
  return request<Response<REDIS_API.RedisInfoVo>>(`/api/redisInfo/${id}`, {
    method: 'GET',
  });
}

// /** 获取当前的用户 GET /api/currentUser */
// export async function currentUser(options?: { [key: string]: any }) {
//   return request<Response<USER_API.CurrentUser>>('/api/user/loginUser', {
//     method: 'GET',
//     ...(options || {}),
//   });
// }

export function getDefaultRedisInfoVo(){
  return {
    id: -5,
    name: "",
    ///redis的主机地址，可以是域名，也可以是ip
    host: "",
    ///redis的端口
    port: 6379,
    username:"",
    password:"",
    ///redis的集群类型，STANDALONE，CLUSTER，SENTINEL
    clusterType: "",
    createTime: "",
    createId: -5,
    updateTime: "",
    updateId: -5,
  };
}

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

/** 实时节点相关信息 POST /api/redisInfo/relatedInfo */
export async function redisInfoFindRelatedInfoRt(body: REDIS_API.RedisInfoRelatedInfoRtDto, options?: { [key: string]: any }) {
  return request<Response<REDIS_API.RedisNodeInfoVo[]>>(`/api/redisInfo/relatedInfoRt`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    data: body,
    ...(options || {}),
  });
}

// /** 获取当前的用户 GET /api/currentUser */
// export async function currentUser(options?: { [key: string]: any }) {
//   return request<Response<USER_API.CurrentUser>>('/api/user/loginUser', {
//     method: 'GET',
//     ...(options || {}),
//   });
// }

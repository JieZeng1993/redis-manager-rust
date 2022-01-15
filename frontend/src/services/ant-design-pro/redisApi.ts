import {request} from 'umi';
import type {Response} from "@/services/ant-design-pro/index";

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

/** 登录接口 POST /api/redisInfo/:id */
export async function redisInfoDeleteById(id: number) {
  return request<Response<REDIS_API.RedisInfoVo>>(`/api/redisInfo/${id}`, {
    method: 'DELETE',
  });
}

/** 登录接口 POST /api/redisInfo/:id */
export async function redisInfoFindBy(params: REDIS_API.RedisInfoVo, props: any) {
  const id = params?.id;
  if (id) {
    const result = await request<REDIS_API.RedisInfoVo>(`/api/redisInfo/${id}`, {
      method: 'GET',
      getResponse: false,
      ...(props || {}),
    });
    return result.data || {};
  } else {
    return {};
  }
}

/** 实时节点相关信息 POST /api/redisInfo/relatedInfo */
export async function redisInfoFindRelatedInfoRt(body: REDIS_API.RedisConnectDto, options?: { [key: string]: any }) {
  if (body.request) {
    return request<Response<REDIS_API.RedisNodeInfoVo[]>>(`/api/redisInfo/relatedInfoRt`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      data: body,
      ...(options || {}),
    });
  } else {
    // const response: Response={success: true, data: []};
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    return {success: true, data: []};
  }
}


/** 实时节点相关信息 POST /api/redisInfo/connectTest */
export async function requestConnectTest(body: REDIS_API.RedisConnectDto, options?: { [key: string]: any }) {
  if (body.request) {
    return request<Response<string>>(`/api/redisInfo/connectTest`, {
      method: 'POST',
      data: body,
      ...(options || {}),
    });
  } else {
    return {success: true, data: "connected"};
  }
}

export async function updateByConnect(body: REDIS_API.RedisConnectDto, options?: { [key: string]: any }) {
  return request<Response<string>>(`/api/redisInfo/updateByConnect`, {
    method: 'POST',
    data: body,
    ...(options || {}),
  });
}

export async function addByConnect(body: REDIS_API.RedisConnectDto, options?: { [key: string]: any }) {
  return request<Response<string>>(`/api/redisInfo/addByConnect`, {
    method: 'POST',
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

// @ts-ignore
/* eslint-disable */
import { request } from 'umi';

/**
 * 用户相关的接口
 */

/** 登录接口 POST /api/user/login */
export async function loginNew(body: USER_API.LoginParams, options?: { [key: string]: any }) {
    return request<USER_API.LoginResult>('/api/user/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        data: body,
        ...(options || {}),
    });
}


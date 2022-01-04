// @ts-ignore
/* eslint-disable */
import {request} from 'umi';
import {Response} from "@/services/ant-design-pro/index";

/**
 * 用户相关的接口
 */

/** 登录接口 POST /api/user/login */
export async function loginNew(body: USER_API.LoginParams, options?: { [key: string]: any }) {
    return request<Response<USER_API.LoginResult>>('/api/user/login', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        data: body,
        ...(options || {}),
    });
}

/** 获取当前的用户 GET /api/currentUser */
export async function currentUser(options?: { [key: string]: any }) {
    return request<Response<USER_API.CurrentUser>>('/api/user/loginUser', {
        method: 'GET',
        ...(options || {}),
    });
}
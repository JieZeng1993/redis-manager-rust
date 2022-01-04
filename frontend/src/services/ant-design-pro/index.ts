// @ts-ignore
/* eslint-disable */
// API 更新时间：
// API 唯一标识：
import * as api from './api';
import * as login from './login';
import * as userApi from './userApi';

export default {
    api,
    login,
    userApi,
};

/**
 * 通用的返回结果
 */
export interface Response<T = any>  {
    success: boolean; // if request is success
        data?: T; // response data
    errorCode?: string; // code for errorType
    msg?: string; // message display to user
    current?: number, //page request valid
    pageSize?: number, //page request valid
    total?: number, //page request valid
}
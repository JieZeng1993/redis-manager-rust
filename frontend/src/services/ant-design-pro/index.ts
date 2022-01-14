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
export interface Response<T = any> {
  success: boolean; // if request is success
  data?: T; // response data
  errorCode?: string; // code for errorType
  msg?: string; // message display to user
  showType?: number;// error display type： 0 silent; 1 message.warn; 2 message.error; 4 notification; 9 page
  current?: number, //page request valid
  pageSize?: number, //page request valid
  total?: number, //page request valid
}

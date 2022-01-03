// @ts-ignore
/* eslint-disable */
/**
 * 用户相关接口的类型
 */
declare namespace USER_API {
    type LoginParams = {
        name?: string;
        password?: string;
        ///暂时没用
        type?: string;
    };

    type LoginResult = {
        id?: number;
        name?: string;
        authorization?: string;
        createTime?: string;
        createId?: number;
        updateTime?: string;
        updateId?: number;
        ///暂时没用
        type?: string;
    }
}
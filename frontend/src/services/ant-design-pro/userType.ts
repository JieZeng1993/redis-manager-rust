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

    type CurrentUser = {
        id?: number;
        name?: string;
        avatar?: string;
        email?: string;
        signature?: string;
        title?: string;
        group?: string;
        tags?: { key?: string; label?: string }[];
        notifyCount?: number;
        unreadCount?: number;
        country?: string;
        access?: string;
        geographic?: {
            province?: { label?: string; key?: string };
            city?: { label?: string; key?: string };
        };
        address?: string;
        phone?: string;
    };
}
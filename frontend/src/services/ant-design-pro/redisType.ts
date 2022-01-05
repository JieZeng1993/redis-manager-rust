// @ts-ignore
/* eslint-disable */
/**
 * REDIS相关接口的类型
 */
declare namespace REDIS_API {
  type RedisInfoPageDto = {
    id?: number;
    page_no?: number;
    /// page有值表示分页查询
    page_size?: number;
    keyword?: string;
    cluster_type?: string,
  };

  type RedisInfoVo = {
    id?: number;
    name?: string;
    ///redis的主机地址，可以是域名，也可以是ip
    host?: string;
    ///redis的端口
    port?: number;
    ///用户名（空表示无需用户名）
    username?: string;
    ///redis的集群类型，STANDALONE，CLUSTER，SENTINEL
    cluster_type?: string;
    createTime?: string;
    createId?: number;
    updateTime?: string;
    updateId?: number;
    // redis_node_infos: Option<Vec<RedisNodeInfoVo>>,
  }
  //
  // type CurrentUser = {
  //   id?: number;
  //   name?: string;
  //   avatar?: string;
  //   email?: string;
  //   signature?: string;
  //   title?: string;
  //   group?: string;
  //   tags?: { key?: string; label?: string }[];
  //   notifyCount?: number;
  //   unreadCount?: number;
  //   country?: string;
  //   access?: string;
  //   geographic?: {
  //     province?: { label?: string; key?: string };
  //     city?: { label?: string; key?: string };
  //   };
  //   address?: string;
  //   phone?: string;
  // };
}

// @ts-ignore
/* eslint-disable */
/**
 * REDIS相关接口的类型
 */
declare namespace REDIS_API {
  type RedisInfoPageDto = {
    current?: number;
    /// page有值表示分页查询
    pageSize?: number;
    id?: number;
    name?: string;
    ///redis的主机地址，可以是域名，也可以是ip
    host?: string;
    ///redis的端口
    port?: number;
    ///用户名（空表示无需用户名）
    username?: string;
    ///redis的集群类型，STANDALONE，CLUSTER，SENTINEL
    clusterType?: string,
    ///接口中没有这个参数
    updateTimeRange: [],
    ///更新时间范围-开始时间
    updateTimeBegin: String,
    ///更新时间范围-结束时间
    updateTimeEnd: String,
  };

  type RedisInfoVo = {
    id?: number;
    name?: string;
    ///redis的主机地址，可以是域名，也可以是ip
    host?: string;
    ///redis的端口
    port?: number;
    username?:string,
    password?:string,
    ///redis的集群类型，STANDALONE，CLUSTER，SENTINEL
    clusterType?: string;
    createTime?: string;
    createId?: number;
    updateTime?: string;
    updateId?: number;
    // redisNodeInfos: RedisNodeInfoVo[],
  }

  type RedisNodeInfoVo = {
    id?: number;
    name?: string;
    ///redis的主机地址，可以是域名，也可以是ip
    host?: string;
    ///redis的端口
    port?: number;
    username?:string,
    password?:string,
    ///redis的集群类型，STANDALONE，CLUSTER，SENTINEL
    clusterType?: string;
    createTime?: string;
    createId?: number;
    updateTime?: number;
    updateId?: number;
  }
}

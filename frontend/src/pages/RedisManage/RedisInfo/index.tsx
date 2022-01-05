import React from "react";
import {PageContainer} from "@ant-design/pro-layout";
import ProTable, {ProColumns} from '@ant-design/pro-table';
import {FormattedMessage, useIntl} from "@@/plugin-locale/localeExports";
import {Button} from "antd";
import {PlusOutlined} from "@ant-design/icons";
import {redisInfoPage} from "@/services/ant-design-pro/redisApi";
import moment from 'moment';

const RedisInfo: React.FC = () => {
    // @ts-ignore
    const redisInfoPageColumns: ProColumns<REDIS_API.RedisInfoVo>[] = [
      {
        title: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.name"
            defaultMessage="名称"
          />
        ),
        dataIndex: 'name',
        tip: 'redis信息名称唯一',
        render: (dom, entity) => {
          return (
            <a
              onClick={() => {
                // setCurrentRow(entity);
                // setShowDetail(true);
              }}
            >
              {dom}
            </a>
          );
        },
      },
      {
        title: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.host"
            defaultMessage="host"
          />
        ),
        dataIndex: 'host',
        tip: 'ip或主机名',
        valueType: 'textarea',
      },
      {
        title: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.port"
            defaultMessage="端口"
          />
        ),
        dataIndex: 'port',
        valueType: 'textarea',
      },
      {
        title: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.clusterType"
            defaultMessage="集群类型"
          />
        ),
        dataIndex: 'clusterType',
        // STANDALONE，CLUSTER，SENTINEL
        valueEnum: {
          "STANDALONE": {
            text: (
              <FormattedMessage id="redisManage.redisInfo.redisInfoVo.clusterType.STANDALONE" defaultMessage="单机"/>
            ),
            status: '单机',
          },
          "CLUSTER": {
            text: (
              <FormattedMessage id="redisManage.redisInfo.redisInfoVo.clusterType.CLUSTER" defaultMessage="集群"/>
            ),
            status: '集群',
          },
          "SENTINEL": {
            text: (
              <FormattedMessage id="redisManage.redisInfo.redisInfoVo.clusterType.SENTINEL" defaultMessage="哨兵"/>
            ),
            status: '哨兵',
          },
        },
      },
      {
        title: '日期范围',
        dataIndex: 'updateTimeRange',
        valueType: 'dateRange',
        hideInTable: true,
        initialValue: [moment(), moment().add(1, 'day')],
      },
      {
        title: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.updateTime"
            defaultMessage="更新时间"
          />
        ),
        hideInSearch: true,
        dataIndex: 'updateTime',
        valueType: 'dateTime',
        // sorter: (a, b) => a.updateTime - b.updateTime,
      },
      {
        title: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.updateId"
            defaultMessage="更新人"
          />
        ),
        hideInSearch: true,
        hideInTable: true,
        dataIndex: 'updateId',
        valueType: 'textarea',
      },
      {
        title: <FormattedMessage id="operate" defaultMessage="操作"/>,
        valueType: 'option',
        render: (_, record) => [
          // <a
          //   key="config"
          //   onClick={() => {
          //     // handleUpdateModalVisible(true);
          //     // setCurrentRow(record);
          //   }}
          // >
          //   <FormattedMessage id="pages.searchTable.config" defaultMessage="Configuration" />
          // </a>,
          // <a key="subscribeAlert" href="https://procomponents.ant.design/">
          //   <FormattedMessage
          //     id="pages.searchTable.subscribeAlert"
          //     defaultMessage="Subscribe to alerts"
          //   />
          // </a>,
        ],
      },
    ]
    //国际化
    const intl = useIntl();
    return (
      <PageContainer>
        <ProTable<REDIS_API.RedisInfoVo, REDIS_API.RedisInfoPageDto>
          headerTitle={intl.formatMessage({
            id: 'redisManage.redisInfo.searchTable.title',
            defaultMessage: 'redis信息列表',
          })}
          rowKey="id"
          search={{
            labelWidth: 120,
          }}
          toolBarRender={() => [
            <Button
              type="primary"
              key="primary"
              onClick={() => {
                // handleModalVisible(true);
              }}
            >
              <PlusOutlined/> <FormattedMessage id="redisManage.redisInfo.new" defaultMessage="New"/>
            </Button>,
          ]}
          request={redisInfoPage}
          columns={redisInfoPageColumns}
        />
      </PageContainer>
    )
      ;
  }
;

export default RedisInfo;

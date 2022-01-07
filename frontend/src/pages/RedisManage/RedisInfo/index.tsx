import React, {useRef, useState} from "react";
import {PageContainer} from "@ant-design/pro-layout";
import type {ActionType, ProColumns} from '@ant-design/pro-table';
import ProTable from '@ant-design/pro-table';
import {Button, Drawer} from "antd";
import {PlusOutlined} from "@ant-design/icons";
import {redisInfoPage} from "@/services/ant-design-pro/redisApi";
import type {ProDescriptionsItemProps} from "@ant-design/pro-descriptions";
import ProDescriptions from "@ant-design/pro-descriptions";
import UpdateForm from "./components/UpdateForm";
import {FormattedMessage, history, useIntl} from 'umi';

const RedisInfo: React.FC = () => {

    //详情展示
    const [showDetail, setShowDetail] = useState<boolean>(false);

    //修改展示
    const [updateModalVisible, handleUpdateModalVisible] = useState<boolean>(false);

    //修改刷新触发
    const actionRef = useRef<ActionType>();

    const [currentRow, setCurrentRow] = useState<REDIS_API.RedisInfoVo>();


    const redisInfoPageColumns: ProColumns<REDIS_API.RedisInfoVo>[] = [
      {
        title: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.name"
            defaultMessage="名称"
          />
        ),
        dataIndex: 'name',
        // @ts-ignore
        tip: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.tip.name"
            defaultMessage="redis info name unique"
          />
        ),
        render: (dom, entity) => {
          return (
            <a
              onClick={() => {
                setCurrentRow(entity);
                setShowDetail(true);
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
        // @ts-ignore
        tip: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.tip.host"
            defaultMessage="ip or host"
          />
        ),
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
        title: (
          <FormattedMessage
            id="redisManage.redisInfo.redisInfoVo.updateTimeRange"
            defaultMessage="更新时间范围"
          />
        ),
        dataIndex: 'updateTimeRange',
        valueType: 'dateRange',
        hideInTable: true,
        //详情的时候不展示
        hideInDescriptions: true,
        // initialValue: [moment(), moment().add(1, 'day')],
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
        //详情的时候不展示
        hideInDescriptions: true,
        dataIndex: 'updateId',
        valueType: 'textarea',
      },
      {
        title: <FormattedMessage id="operate" defaultMessage="operate"/>,
        valueType: 'option',
        render: (_, record) => [
          <a
            key="config"
            onClick={() => {
              // handleUpdateModalVisible(true);
              // setCurrentRow(record);
              history.push(`/redisManage/redisInfo/update/${record.id}`)
            }}
          >
            <FormattedMessage id="modify" defaultMessage="修改"/>
          </a>,
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
          actionRef={actionRef}
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
              <PlusOutlined/> <FormattedMessage id="new" defaultMessage="New"/>
            </Button>,
          ]}
          request={redisInfoPage}
          columns={redisInfoPageColumns}
        />

        <UpdateForm
          onSubmit={async (value) => {
            // const success = await handleUpdate(value);
            console.log(value);
            const success = true;
            if (success) {
              handleUpdateModalVisible(false);
              setCurrentRow(undefined);
              if (actionRef.current) {
                actionRef.current.reload();
              }
            }
          }}
          onCancel={() => {
            handleUpdateModalVisible(false);
            if (!showDetail) {
              setCurrentRow(undefined);
            }
          }}
          visible={updateModalVisible}
          current={currentRow || {}}
          done={false}/>

        <Drawer
          width={600}
          visible={showDetail}
          onClose={() => {
            setCurrentRow(undefined);
            setShowDetail(false);
          }}
          closable={false}
        >
          {currentRow?.name && (
            <ProDescriptions<API.RuleListItem>
              column={2}
              title={currentRow?.name}
              request={async () => ({
                data: currentRow || {},
              })}
              params={{
                id: currentRow?.name,
              }}
              columns={redisInfoPageColumns as ProDescriptionsItemProps<REDIS_API.RedisInfoVo>[]}
            />
          )}
        </Drawer>
      </PageContainer>
    )
      ;
  }
;

export default RedisInfo;

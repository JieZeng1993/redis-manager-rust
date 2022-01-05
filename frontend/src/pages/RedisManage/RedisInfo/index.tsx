import React from "react";
import {PageContainer} from "@ant-design/pro-layout";
import ProTable, {ProColumns} from '@ant-design/pro-table';
import {FormattedMessage, useIntl} from "@@/plugin-locale/localeExports";
import {Button} from "antd";
import {PlusOutlined} from "@ant-design/icons";
import {redisInfoPage} from "@/services/ant-design-pro/redisApi";

const RedisInfo: React.FC = () => {
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
};

export default RedisInfo;

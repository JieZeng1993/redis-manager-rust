import React from "react";
import {PageContainer} from "@ant-design/pro-layout";
import ProTable from '@ant-design/pro-table';
import {useIntl} from "@@/plugin-locale/localeExports";

const RedisInfo: React.FC = () => {
  //国际化
  const intl = useIntl();
  return (
    <PageContainer>
      <ProTable< >

      </ProTable>
    </PageContainer>
  );
};

export default RedisInfo;

import React, {useRef, useState} from 'react';
import type {FormInstance} from 'antd';
import {Button, Card, Descriptions, message, Result} from 'antd';
import {PageContainer} from '@ant-design/pro-layout';
import ProForm, {ProFormDigit, ProFormText, StepsForm} from '@ant-design/pro-form';
import styles from './style.less';
import {history, useIntl, useParams} from 'umi';
import {
  addByConnect,
  redisInfoFindBy,
  redisInfoFindRelatedInfoRt,
  requestConnectTest,
  updateByConnect
} from "@/services/ant-design-pro/redisApi";
import type {ProColumns} from "@ant-design/pro-table";
import ProTable from "@ant-design/pro-table";
import {FormattedMessage} from "@@/plugin-locale/localeExports";
import {toNumber} from "lodash";
import {INVALID_NUMBER} from "@/services/ant-design-pro/api";

const StepDescriptions: React.FC<{
  stepData: REDIS_API.RedisInfoVo | undefined;
  bordered?: boolean;
}> = ({stepData, bordered}) => {
  //国际化
  const intl = useIntl();

  if (stepData) {
    const {name, host, port} = stepData;
    return (
      <Descriptions column={1} bordered={bordered}>
        <Descriptions.Item label={intl.formatMessage({
          id: 'redisManage.redisInfo.redisInfoVo.name',
          defaultMessage: 'name',
        })}> {name}</Descriptions.Item>
        <Descriptions.Item label={intl.formatMessage({
          id: 'redisManage.redisInfo.redisInfoVo.host',
          defaultMessage: 'host',
        })}> {host}</Descriptions.Item>
        <Descriptions.Item label={intl.formatMessage({
          id: 'redisManage.redisInfo.redisInfoVo.port',
          defaultMessage: 'port',
        })}> {port}</Descriptions.Item>
      </Descriptions>
    );
  } else {
    return (
      <Descriptions column={1} bordered={bordered}>
        <Descriptions.Item> {intl.formatMessage({
          id: 'noData',
          defaultMessage: 'no_data',
        })}</Descriptions.Item>
      </Descriptions>
    )
  }
};

const StepResult: React.FC<{
  onFinish: () => Promise<void>;
}> = (props) => {

  return (
    <Result
      status="success"
      title="操作成功"
      subTitle="返回查询redis信息"
      extra={
        <>
          <Button type="primary" onClick={props.onFinish}>
            返回
          </Button>
          {/*<Button>查看账单</Button>*/}
        </>
      }
      className={styles.result}
    >
      {props.children}
    </Result>
  );
};

const RedisInfoUpdate: React.FC<Record<string, any>> = () => {
  //国际化
  const intl = useIntl();
  // @ts-ignore
  const {id} = useParams();
  const infoParams = {id: id};

  const [stepData, setStepData] = useState<REDIS_API.RedisInfoVo>();
  const [nodeInfoParams, setNodeInfoParams] = useState<REDIS_API.RedisConnectDto>();

  //保存最新的值，方便保存使用
  let nodeInfoData: REDIS_API.RedisNodeInfoVo[] = [];

  const [current, setCurrent] = useState(0);

  const [loadings, setLoading] = useState({});

  const formRef = useRef<FormInstance>();

  function onCurrentChange(number: number) {
    //number是从0开始的，
    if (number == 1) {
      setNodeInfoParams({
        // @ts-ignore
        id: stepData.id,
        // @ts-ignore
        name: stepData.name,
        ///redis的主机地址，可以是域名，也可以是ip
        // @ts-ignore
        host: stepData.host,
        ///redis的端口
        // @ts-ignore
        port: stepData.port,
        // @ts-ignore
        username: stepData.username,
        // @ts-ignore
        password: stepData.password,
        request: true
      });
    } else {
      setNodeInfoParams({request: false});
    }
    setCurrent(number);
  }

  //处理新数据没有id的情况
  function redisInfoFindRelatedInfoDeal(data: REDIS_API.RedisNodeInfoVo[]) {
    nodeInfoData = data;
    data.forEach(redisInfoFindRelatedInfoItem => {

      if (redisInfoFindRelatedInfoItem?.slotFrom == INVALID_NUMBER) {
        redisInfoFindRelatedInfoItem.slotFrom = undefined;
      }

      if (redisInfoFindRelatedInfoItem?.slotTo == INVALID_NUMBER) {
        redisInfoFindRelatedInfoItem.slotTo = undefined;
      }

      if (redisInfoFindRelatedInfoItem?.id) {
        return;
      }

      // @ts-ignore
      redisInfoFindRelatedInfoItem.id = (redisInfoFindRelatedInfoItem?.host || "") + (redisInfoFindRelatedInfoItem?.port || "");
    });
    return data;
  }

  async function saveRedisNodeInfo() {
    if (!nodeInfoData || nodeInfoData.length === 0) {
      message.error("节点信息未加载成功，请检查相关信息是否正确，加载节点信息成功后，再保存")
      return false;
    }
    const requestBody = nodeInfoParams || {};
    //存储逻辑
    try {
      let result;
      if (requestBody.id) {
        result = await updateByConnect(requestBody || {});
      } else {
        result = await addByConnect(requestBody || {});
      }

      if (result.success) {
        if (requestBody.id) {
          message.success('更新成功');
        } else {
          message.success('新增成功');
        }
      }
      return result.success;
    } catch (e) {
      console.log(e);
      return false;
    }
  }

  function connectTest(redisNodeInfo: REDIS_API.RedisNodeInfoVo) {
    const loading_key = redisNodeInfo.id||"";
    if (loadings[loading_key]) {
      return;
    }

    // @ts-ignore
    setLoading(({oldLoadings})=>{
      const newLoadings = {...oldLoadings};
      newLoadings[loading_key] = true;
      console.log(JSON.stringify(newLoadings));
      return newLoadings;
    });

    redisNodeInfo.connecting = true;

    const request: REDIS_API.RedisConnectDto = {
      id: stepData?.id,
      host: redisNodeInfo.host,
      port: redisNodeInfo.port,
      username: stepData?.username,
      password: stepData?.password,
      request: true,
    }

    requestConnectTest(request).then(()=>{
      message.success("连接成功");
    }).finally(() => {
      // @ts-ignore
      setLoading(({oldLoadings})=>{
        const newLoadings = {...oldLoadings};
        console.log(JSON.stringify(newLoadings));
        delete newLoadings[loading_key];
        console.log(JSON.stringify(newLoadings));
        return newLoadings;
      });
    })
  }

  const redisNodeInfoVoColumns: ProColumns<REDIS_API.RedisNodeInfoVo>[] = [
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
      valueType: 'text',
    },
    {
      title: (
        <FormattedMessage
          id="redisManage.redisInfo.redisNodeInfoVo.nodeId"
          defaultMessage="nodeId"
        />
      ),
      copyable: true,
      dataIndex: 'nodeId',
      valueType: 'textarea',
    },
    {
      title: (
        <FormattedMessage
          id="redisManage.redisInfo.redisNodeInfoVo.masterId"
          defaultMessage="masterId"
        />
      ),
      copyable: true,
      dataIndex: 'masterId',
      valueType: 'textarea',
    },
    {
      title: (
        <FormattedMessage
          id="redisManage.redisInfo.redisNodeInfoVo.nodeRole"
          defaultMessage="nodeRole"
        />
      ),
      dataIndex: 'nodeRole',
      valueEnum: {
        "MASTER": {
          text: (
            <FormattedMessage id="redisManage.redisInfo.redisNodeInfoVo.nodeRole.master" defaultMessage="MASTER"/>
          ),
        },
        "SLAVE": {
          text: (
            <FormattedMessage id="redisManage.redisInfo.redisNodeInfoVo.nodeRole.slave" defaultMessage="SLAVE"/>
          ),
        },
      },
    },
    {
      title: (
        <FormattedMessage
          id="redisManage.redisInfo.redisNodeInfoVo.slot"
          defaultMessage="slot"
        />
      ),
      dataIndex: 'slotFromAndSlotTo',
      valueType: 'text',
      render: (_, record) => (
        <span>
          {record.slotFrom}-{record.slotTo}
        </span>
      ),
    },
    {
      title: (
        <FormattedMessage
          id="redisManage.redisInfo.redisNodeInfoVo.nodeStatus"
          defaultMessage="nodeStatus"
        />
      ),
      dataIndex: 'nodeStatus',
      valueEnum: {
        "CONNECTED": {
          text: (
            <FormattedMessage id="redisManage.redisInfo.redisNodeInfoVo.nodeStatus.connected"
                              defaultMessage="CONNECTED"/>
          ),
          status: 'CONNECTED',
        },
      },
    },
    {
      title: (
        <FormattedMessage
          id="redisManage.redisInfo.redisInfoVo.updateTime"
          defaultMessage="更新时间"
        />
      ),
      hideInSearch: true,
      hideInTable: true,
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
        <Button
          type={'link'}
          key="redisConnectTestButton"
          loading={loadings[record.id || ""]}
          onClick={() => {
            connectTest(record)
          }}
        >
          <FormattedMessage id="connection" defaultMessage="connection"/>
          <FormattedMessage id="test" defaultMessage="test"/>
        </Button>,
        // <a key="subscribeAlert" href="https://procomponents.ant.design/">
        //   <FormattedMessage
        //     id="pages.searchTable.subscribeAlert"
        //     defaultMessage="Subscribe to alerts"
        //   />
        // </a>,
      ],
    },
  ]


  return (
    <PageContainer title={intl.formatMessage({
      id: 'redisManage.redisInfo.modify.title',
      defaultMessage: 'name',
    }) +
    (id ? intl.formatMessage({
        id: 'edit',
      }) :
      intl.formatMessage({
        id: 'add',
      }))
    }
                   extra={[
                     <Button key="back" onClick={() => history.goBack()}>返回</Button>
                   ]}
    >
      <Card bordered={false}>
        <StepsForm
          current={current}
          onCurrentChange={onCurrentChange}
          submitter={{
            render: (props, dom) => {
              // if (props.step === 0){
              //   return (
              //     <Button type="primary" onClick={()=>props.onSubmit?.()}>
              //       下一步
              //     </Button>
              //   );
              // }

              if (props.step === 2) {
                return null;
              }
              return dom;
            },
          }}
        >
          <StepsForm.StepForm<REDIS_API.RedisInfoVo>
            request={redisInfoFindBy}
            params={infoParams}
            title={intl.formatMessage({
              id: 'rule.pleaseFill',
              defaultMessage: 'please fill',
            }) + intl.formatMessage({
              id: 'redisManage.redisInfo.modify.title',
              defaultMessage: 'redis info',
            })}
            onFinish={async (values) => {
              if (infoParams.id) {
                values.id = toNumber(infoParams.id);
              }
              setStepData(values);
              return true;
            }}
          >
            <ProFormText
              label={intl.formatMessage({
                id: 'redisManage.redisInfo.redisInfoVo.name',
                defaultMessage: 'name',
              })}
              width="md"
              name="name"
              rules={[{
                required: true, message: intl.formatMessage({
                  id: 'rule.pleaseInput',
                  defaultMessage: 'please input',
                }) + intl.formatMessage({
                  id: 'redisManage.redisInfo.redisInfoVo.name',
                  defaultMessage: 'name',
                })
              }]}
              placeholder={intl.formatMessage({
                id: 'rule.pleaseInput',
                defaultMessage: 'please input',
              }) + intl.formatMessage({
                id: 'redisManage.redisInfo.redisInfoVo.name',
                defaultMessage: 'name',
              })}
            />

            <ProForm.Group label={intl.formatMessage({
              id: 'redisManage.redisInfo.redisInfoVo.host',
              defaultMessage: 'host',
            }) + ":" + intl.formatMessage({
              id: 'redisManage.redisInfo.redisInfoVo.port',
              defaultMessage: 'port',
            })} size={8}>
              <ProFormText
                name="host"
                rules={[
                  {
                    required: true, message: intl.formatMessage({
                      id: 'rule.pleaseInput',
                      defaultMessage: 'please input ',
                    }) + intl.formatMessage({
                      id: 'redisManage.redisInfo.redisInfoVo.tip.host',
                      defaultMessage: 'ip or host',
                    })
                  },
                ]}
                placeholder={intl.formatMessage({
                  id: 'redisManage.redisInfo.redisInfoVo.tip.host',
                  defaultMessage: 'ip or host',
                })}
              />
              <ProFormDigit
                name="port"
                //整数
                fieldProps={{precision: 0}}
                rules={[
                  {
                    required: true, message: intl.formatMessage({
                      id: 'rule.pleaseInput',
                      defaultMessage: 'please input ',
                    }) + intl.formatMessage({
                      id: 'redisManage.redisInfo.redisInfoVo.port',
                      defaultMessage: 'port',
                    })
                  },
                  {
                    type: 'integer', message: intl.formatMessage({
                      id: 'redisManage.redisInfo.redisInfoVo.port',
                      defaultMessage: 'port',
                    }) + intl.formatMessage({
                      id: 'requireInteger',
                      defaultMessage: ' is integer',
                    })
                  },
                ]}
                placeholder="端口"
              />
            </ProForm.Group>

            <ProFormText
              label={intl.formatMessage({
                id: 'redisManage.redisInfo.redisInfoVo.username',
                defaultMessage: 'username',
              })}
              tooltip={intl.formatMessage({
                id: 'noIsNoNeedInput',
                defaultMessage: 'no is no need input',
              })}
              width="md"
              name="username"
              placeholder={intl.formatMessage({
                id: 'rule.pleaseInput',
                defaultMessage: 'please input ',
              }) + intl.formatMessage({
                id: 'redisManage.redisInfo.redisInfoVo.username',
                defaultMessage: 'username',
              })}
            />

            <ProFormText
              label={intl.formatMessage({
                id: 'redisManage.redisInfo.redisInfoVo.password',
                defaultMessage: 'password',
              })}
              tooltip={intl.formatMessage({
                id: 'noIsNoNeedInput',
                defaultMessage: 'no is no need input',
              })}
              width="md"
              name="password"
              placeholder={intl.formatMessage({
                id: 'rule.pleaseInput',
                defaultMessage: 'please input ',
              }) + intl.formatMessage({
                id: 'redisManage.redisInfo.redisInfoVo.password',
                defaultMessage: 'password',
              })}
            />
          </StepsForm.StepForm>

          <StepsForm.StepForm title="确认redis相关信息"
                              onFinish={saveRedisNodeInfo}
          >
            <ProTable<REDIS_API.RedisNodeInfoVo, REDIS_API.RedisConnectDto>
              params={nodeInfoParams}
              pagination={{position: []}}
              headerTitle={intl.formatMessage({
                id: 'redisManage.redisInfo.searchTable.title',
                defaultMessage: 'redis信息列表',
              })}
              // actionRef={actionRef}
              rowKey="id"
              search={false}
              toolBarRender={false}
              request={redisInfoFindRelatedInfoRt}
              postData={redisInfoFindRelatedInfoDeal}
              columns={redisNodeInfoVoColumns}
            />

          </StepsForm.StepForm>
          <StepsForm.StepForm title="完成">
            <StepResult
              onFinish={async () => {
                formRef.current?.resetFields();
                history.goBack();
              }}
            >
              <StepDescriptions stepData={stepData}/>
            </StepResult>
          </StepsForm.StepForm>
        </StepsForm>
      </Card>
    </PageContainer>
  );
};

export default RedisInfoUpdate;

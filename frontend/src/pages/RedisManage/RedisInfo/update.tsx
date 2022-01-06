import React, {useEffect, useRef, useState} from 'react';
import type {FormInstance} from 'antd';
import {Alert, Button, Card, Descriptions, Divider, Result} from 'antd';
import {PageContainer} from '@ant-design/pro-layout';
import ProForm, {ProFormDigit, ProFormText, StepsForm} from '@ant-design/pro-form';
import styles from './style.less';
import {useIntl, useParams} from 'umi';
import { redisInfoFindById, redisInfoFindRelatedInfoRt} from "@/services/ant-design-pro/redisApi";

const StepDescriptions: React.FC<{
  stepData: REDIS_API.RedisInfoVo;
  bordered?: boolean;
}> = ({stepData, bordered}) => {
  //国际化
  const intl = useIntl();

  if(stepData){
    const {name, host, port, username, password, clusterType} = stepData ;
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
        <Descriptions.Item label={intl.formatMessage({
          id: 'redisManage.redisInfo.redisInfoVo.username',
          defaultMessage: 'username',
        })}> {username}</Descriptions.Item>
        <Descriptions.Item label={intl.formatMessage({
          id: 'redisManage.redisInfo.redisInfoVo.password',
          defaultMessage: 'password',
        })}> {password}</Descriptions.Item>
        <Descriptions.Item label={intl.formatMessage({
          id: 'redisManage.redisInfo.redisInfoVo.clusterType',
          defaultMessage: 'clusterType',
        })}> {clusterType}</Descriptions.Item>
      </Descriptions>
    );
  }else {
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
      subTitle="预计两小时内到账"
      extra={
        <>
          <Button type="primary" onClick={props.onFinish}>
            再转一笔
          </Button>
          <Button>查看账单</Button>
        </>
      }
      className={styles.result}
    >
      {props.children}
    </Result>
  );
};

const RedisInfoUpdate: React.FC<Record<string, any>> = () => {
  // @ts-ignore
  const {id} = useParams();

  const [stepData, setStepData] = useState<REDIS_API.RedisInfoVo>();
  const [redisNodeInfoVos, setRedisNodeInfoVos] = useState<REDIS_API.RedisNodeInfoVo[]>();
  const [current, setCurrent] = useState(0);
  const formRef = useRef<FormInstance>();

  useEffect(() => {
    redisInfoFindById(id).then((result) => {
      const data = result?.data;
      setStepData(data)
    }).catch(() => {
      setStepData(undefined)
    })
  },[id]);

  console.log("RedisInfoUpdate INIT ");

  function onCurrentChange(number: number) {
    //number是从0开始的，
    if (number == 1) {
      //需要加载相关节点信息
      redisInfoFindRelatedInfoRt({
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
      }).then((response) => {
        setRedisNodeInfoVos(response.data);
      }).catch(e=>{
        console.log(e);
        setRedisNodeInfoVos([]);
      }).finally(()=>{
        setCurrent(number);
      })
    }
  }


  //国际化
  const intl = useIntl();
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
    }>
      <Card bordered={false}>
        <StepsForm
          current={current}
          onCurrentChange={onCurrentChange}
          submitter={{
            render: (props, dom) => {
              if (props.step === 2) {
                return null;
              }
              return dom;
            },
          }}
        >
          <StepsForm.StepForm<REDIS_API.RedisInfoVo>
            formRef={formRef}
            title={intl.formatMessage({
              id: 'rule.pleaseFill',
              defaultMessage: 'please fill',
            }) + intl.formatMessage({
              id: 'redisManage.redisInfo.modify.title',
              defaultMessage: 'redis info',
            })}
            initialValues={stepData}
            onFinish={async (values) => {
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
                placeholder="6379"
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

          <StepsForm.StepForm title={intl.formatMessage({
            id: 'confirm',
            defaultMessage: 'confirm ',
          }) + "redis" + intl.formatMessage({
            id: 'relatedInformation',
            defaultMessage: ' related information',
          })}>
            <div className={styles.result}>
              <Alert
                closable
                showIcon
                message="确认转账后，资金将直接打入对方账户，无法退回。"
                style={{marginBottom: 24}}
              />
              <StepDescriptions stepData={stepData ? stepData : {}} bordered/>
              <Divider style={{margin: '24px 0'}}/>
              <ProFormText.Password
                label="支付密码"
                width="md"
                name="password"
                required={false}
                rules={[{required: true, message: '需要支付密码才能进行支付'}]}
              />
            </div>
          </StepsForm.StepForm>
          <StepsForm.StepForm title="完成">
            <StepResult
              onFinish={async () => {
                setCurrent(0);
                formRef.current?.resetFields();
              }}
            >
              <StepDescriptions stepData={stepData}/>
            </StepResult>
          </StepsForm.StepForm>
        </StepsForm>
        <Divider style={{margin: '40px 0 24px'}}/>
        <div className={styles.desc}>
          <h3>说明</h3>
          <h4>转账到支付宝账户</h4>
          <p>
            如果需要，这里可以放一些关于产品的常见问题说明。如果需要，这里可以放一些关于产品的常见问题说明。如果需要，这里可以放一些关于产品的常见问题说明。
          </p>
          <h4>转账到银行卡</h4>
          <p>
            如果需要，这里可以放一些关于产品的常见问题说明。如果需要，这里可以放一些关于产品的常见问题说明。如果需要，这里可以放一些关于产品的常见问题说明。
          </p>
        </div>
      </Card>
    </PageContainer>
  );
};

export default RedisInfoUpdate;

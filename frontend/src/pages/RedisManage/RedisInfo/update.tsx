import React, {useEffect, useRef, useState} from 'react';
import type {FormInstance} from 'antd';
import {Alert, Button, Card, Descriptions, Divider, Result} from 'antd';
import {PageContainer} from '@ant-design/pro-layout';
import ProForm, {ProFormDigit, ProFormSelect, ProFormText, StepsForm} from '@ant-design/pro-form';
import styles from './style.less';
import {useIntl, useParams} from 'umi';
import {getDefaultRedisInfoVo, redisInfoFindById} from "@/services/ant-design-pro/redisApi";


const StepDescriptions: React.FC<{
  stepData: REDIS_API.RedisInfoVo;
  bordered?: boolean;
}> = ({stepData, bordered}) => {
  //国际化
  const intl = useIntl();

  const {name, host, port, username, password, clusterType} = stepData?stepData:getDefaultRedisInfoVo();
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

  const defaultVal = getDefaultRedisInfoVo();
  const [stepData, setStepData] = useState<REDIS_API.RedisInfoVo>();
  useEffect(()=>{
    redisInfoFindById(id).then((result) => {
      const data = result?.data;
      setStepData(data )
    }).catch(() => {
      setStepData(undefined)
    })
  },[]);

  console.log("RedisInfoUpdate INIT ");
  const [current, setCurrent] = useState(0);
  const formRef = useRef<FormInstance>();
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
          onCurrentChange={setCurrent}
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
            <ProFormSelect
              label="付款账户"
              width="md"
              name="payAccount"
              rules={[{required: true, message: '请选择付款账户'}]}
              valueEnum={{
                'ant-design@alipay.com': 'ant-design@alipay.com',
              }}
            />

            <ProForm.Group title="收款账户" size={8}>
              <ProFormSelect
                name="receiverMode"
                rules={[{required: true, message: '请选择付款账户'}]}
                valueEnum={{
                  alipay: '支付宝',
                  bank: '银行账户',
                }}
              />
              <ProFormText
                name="receiverAccount"
                rules={[
                  {required: true, message: '请输入收款人账户'},
                  {type: 'email', message: '账户名应为邮箱格式'},
                ]}
                placeholder="test@example.com"
              />
            </ProForm.Group>
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
            <ProFormDigit
              label="转账金额"
              name="amount"
              width="md"
              rules={[
                {required: true, message: '请输入转账金额'},
                {
                  pattern: /^(\d+)((?:\.\d+)?)$/,
                  message: '请输入合法金额数字',
                },
              ]}
              placeholder="请输入金额"
              fieldProps={{
                prefix: '￥',
              }}
            />
          </StepsForm.StepForm>

          <StepsForm.StepForm title="确认转账信息">
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

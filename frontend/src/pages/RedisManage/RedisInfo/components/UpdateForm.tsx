import type {FC} from 'react';
import {ModalForm, ProFormDateTimePicker, ProFormSelect, ProFormText, ProFormTextArea,} from '@ant-design/pro-form';
import styles from '../style.less';
import {Button, Result} from 'antd';
import {useIntl} from "@@/plugin-locale/localeExports";
// import {FormattedMessage} from "@@/plugin-locale/localeExports";
// import React from "react";

type OperationModalProps = {
  done: boolean;
  visible: boolean;
  current: Partial<REDIS_API.RedisInfoVo> | undefined;
  onCancel: () => void;
  onSubmit: (value: REDIS_API.RedisInfoVo) => void;
};

const UpdateForm: FC<OperationModalProps> = (props) => {
  const intl = useIntl();

  const {done, visible, current, onCancel, onSubmit, children} = props;
  if (!visible) {
    return null;
  }
  return (
    <ModalForm<REDIS_API.RedisInfoVo>
      visible={visible}
      title={done ? null : (intl.formatMessage({
          id: 'redisManage.redisInfo.modal.title',
          defaultMessage: 'redis info',
        }) +
        (current ? intl.formatMessage({
          id: 'edit',
          defaultMessage: 'edit',
        }) : intl.formatMessage({
          id: 'add',
          defaultMessage: 'add',
        })))}
      className={styles.standardListForm}
      width={640}
      onFinish={async (values) => {
        onSubmit(values);
      }}
      initialValues={current}
      submitter={{
        render: (_, dom) => (done ? null : dom),
      }}
      trigger={<>{children}</>}
      modalProps={{
        onCancel: () => onCancel(),
        destroyOnClose: true,
        bodyStyle: done ? {padding: '72px 0'} : {},
      }}
    >
      {!done ? (
        <>
          <ProFormText
            name="title"
            label={intl.formatMessage({
              id: 'redisManage.redisInfo.redisInfoVo.name',
              defaultMessage: 'name',
            })}
            rules={[{
              required: true, message: intl.formatMessage({
                  id: 'rule.pleaseInput',
                  defaultMessage: 'please input',
                })
                + intl.formatMessage({
                  id: 'redisManage.redisInfo.redisInfoVo.name',
                  defaultMessage: 'name',
                })
            }]}
            placeholder={intl.formatMessage({
              id: 'rule.pleaseInput',
              defaultMessage: 'please input',
            })}
          />
          <ProFormDateTimePicker
            name="createdAt"
            label="????????????"
            rules={[{required: true, message: '?????????????????????'}]}
            fieldProps={{
              style: {
                width: '100%',
              },
            }}
            placeholder="?????????"
          />
          <ProFormSelect
            name="owner"
            label="???????????????"
            rules={[{required: true, message: '????????????????????????'}]}
            options={[
              {
                label: '?????????',
                value: 'xiao',
              },
              {
                label: '?????????',
                value: 'mao',
              },
            ]}
            placeholder="??????????????????"
          />
          <ProFormTextArea
            name="subDescription"
            label="????????????"
            rules={[{message: '?????????????????????????????????????????????', min: 5}]}
            placeholder="???????????????????????????"
          />
        </>
      ) : (
        <Result
          status="success"
          title={intl.formatMessage({
            id: 'operateSuccess',
            defaultMessage: '????????????',
          })}
          subTitle="????????????????????????????????????????????????????????????"
          extra={
            <Button type="primary" onClick={onCancel}>
              ?????????
            </Button>
          }
          className={styles.formResult}
        />
      )}
    </ModalForm>
  );
};

export default UpdateForm;

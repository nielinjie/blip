import { useState, useCallback } from 'react';
import { Form, Input, PageHeader, Divider, Layout, Col, Row } from 'antd';
const { TextArea } = Input;
import { Typography } from 'antd';

const { Title } = Typography;

import {wasmAdapter} from '../w'


const layout = {
  labelCol: {
    span: 8,
  },
  wrapperCol: {
    span: 8,
  },
};
const tailLayout = {
  wrapperCol: {
    offset: 8,
    span: 8,
  },
};
function onFormChange(changedValues: object, allValues: any): void {}

const Demo = () => {
  const [form] = Form.useForm();
  const [codes, setCodes] = useState(['xxxxx', 'x']);
  const itemChange = useCallback((changedValues: object, allValues: any) => {
    console.log(changedValues);
    if (wasmAdapter) {
      let code = wasmAdapter.compute({
        author: allValues.author,
        body: allValues.body,
        time: new Date().getTime(),
        mask: allValues.mask || '1',
      });
      console.log(code);
      setCodes(code);
    }
  }, []);

  return (
    <>
      <PageHeader title="Blip" subTitle="find your blip in the universe." />
      <Form
        {...layout}
        form={form}
        name="control-hooks"
        initialValues={{ author: 'God', mask: '1' }}
        onValuesChange={itemChange}
      >
        <Form.Item
          name="author"
          label="Author"
          rules={[
            {
              required: true,
            },
          ]}
        >
          <Input disabled />
        </Form.Item>
        <Form.Item
          name="body"
          label="Body"
          rules={[
            {
              required: true,
            },
          ]}
        >
          <TextArea rows={4} />
        </Form.Item>
        <Form.Item
          name="mask"
          label="Mask"
          rules={[
            {
              required: true,
            },
          ]}
        >
          <Input />
        </Form.Item>
      </Form>
      <Divider className="divider">Code</Divider>
      <Layout>
        <Row justify="center">
          <Col>
            <Title level={5}>{codes[0]}</Title>
          </Col>
        </Row>
        <Row justify="center">
          <Col>
            <Title>{codes[1]}</Title>
          </Col>
        </Row>
      </Layout>
    </>
  );
};

export default Demo;

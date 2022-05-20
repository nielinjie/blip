import { Typography } from 'antd';
const Text = Typography.Text;
export default function $({ code }) {
  const { Ok, Err } = code[1];
  if (Ok) {
    return (<>
      {Ok.map((pair) => {
        if (pair[1]) {
          return <Text style={{ color: 'red' }}>{pair[0]}</Text>;
        } else {
          return <Text>{pair[0]}</Text>;
        }
      })}
    </>);
  }
  if (Err) {
    return(<Text>{Err}</Text>);
  }
  return <></>
}

import React, {useState} from "react";
import {PageContainer} from "@ant-design/pro-layout";
import Terminal, { ColorMode, LineType } from 'react-terminal-ui';

const RedisQuery: React.FC<Record<string, any>> = () => {
  const [terminalLineData, setTerminalLineData] = useState([
    {type: LineType.Output, value: 'Welcome to the Redis Terminal!'},
    {type: LineType.Input, value: 'connecting'},
  ]);

  return (
    <PageContainer title="cli">
      <div className="container">
        <Terminal name='React Terminal Usage Example'
                  colorMode={ ColorMode.Light }
                  lineData={ terminalLineData }
                  onInput={ (terminalInput: any) => console.log(`New terminal input received: '${ terminalInput }'`) }/>
      </div>
    </PageContainer>
  )
}

export default RedisQuery;

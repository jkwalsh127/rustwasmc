import React, { useEffect, useState } from 'react';
import init, { add_numbers } from "./wasm/pkg/wasm";

function App() {
  const [ans, setAns] = useState(0);

  useEffect(() => {
    init().then(() => {
      setAns(add_numbers(1,2));
    })
  }, []);

  return (
    <div className="app">
      <div className="card">
        <p>1 + 1 = {ans}</p>
      </div>
    </div>
  );
}

export default App;

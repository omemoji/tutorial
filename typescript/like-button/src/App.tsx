import React, {useState} from 'react';
import logo from './logo.svg';
import './App.css';

function App() {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          TypeScriptはいいぞ
        </p>
        <LinkButton/>
      </header>
    </div>
  );
}

function LinkButton(){
  const [count,setCount] = useState(999);
  const handleClick = () => {
    setCount(count + 1);
  };
  return <span className="likeButton" onClick={handleClick}> ♥ {count}</span>
}

export default App;
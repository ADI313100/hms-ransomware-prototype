
import React from 'react';
import ReactDOM from 'react-dom/client';
import './style.css';
function App(){
 return <div style={{fontFamily:'Arial',padding:30,background:'#0f172a',color:'white',minHeight:'100vh'}}>
  <h1>Hospital Ransomware Dashboard</h1>
  <p>Backend: http://localhost:8080</p>
  <p>AI API: http://localhost:8001/risk</p>
 </div>;
}
ReactDOM.createRoot(document.getElementById('root')).render(<App/>);

import './reset.css';
import './style.css';
import { parsePathByTemplate } from './router';

console.log(parsePathByTemplate("/test/path/:param1/lol/:param2", "/test/path/1/lol/abobus"));
console.log(parsePathByTemplate("/test", "/test"));

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
  <div>
  </div>
`

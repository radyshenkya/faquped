// import './reset.css';
import './style.css';
import { Router } from './router';
import { routes, error_route } from './routes';

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
    <nav>
        <a href="/" data-router>index</a>
        <a href="/counter" data-router>counter</a>
        <a href="/posts/123" data-router>post</a>
        <a href="/error" data-router>error</a>
    </nav>
    <div id="router">
    </div>
`;

let router = new Router(routes, error_route);
router.route(window.location.pathname);
router.updateClickEvents();

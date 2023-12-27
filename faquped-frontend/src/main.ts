import './style.css';
import { Router } from './router';
import { routes, error_route } from './routes';

document.querySelector<HTMLDivElement>('#app')!.innerHTML = `
    <section class="flex flex-col min-h-screen">
        <div class="" id="router"></div>
        <footer class="container flex mt-auto pb-2 justify-evenly">
            <a class="hover:underline font-content text-xl text-slate-600" href="https://github.com/radyshenkya/faquped" target="_blank">
                This project on <span class="text-slate-800">Github</span>
            </a>
        </footer>
    </section>
`;

let router = new Router(routes, error_route);
router.route(window.location.pathname);

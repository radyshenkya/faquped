import HttpStatusCode from "./http_status_codes";
import { RouteError, RouteHandlerProps } from "./router";

export const routes = {
    "/": (_props: RouteHandlerProps) => {
        document.querySelector<HTMLDivElement>('#router')!.innerHTML = `<h1>Index Page</h1>`;
    },
    "/about": (_props: RouteHandlerProps) => {
        document.querySelector<HTMLDivElement>('#router')!.innerHTML = `
            <h1>About Page</h1>
        `;
    },
    "/counter": (_props: RouteHandlerProps) => {
        let counter = 0;

        document.querySelector<HTMLDivElement>('#router')!.innerHTML = `
            <h1>Page with counter</h1>
            <hr/>
            <button id="counter-button">count is ${counter}</button>
        `;

        let onCounterClick = () => {
            counter += 1;
            document.querySelector<HTMLButtonElement>('#counter-button')!.innerHTML = `count is ${counter}`;
        };

        document.querySelector<HTMLButtonElement>('#counter-button')!.onclick = onCounterClick;
    },
    "/posts/:post_id": (props: RouteHandlerProps) => {
        document.querySelector<HTMLDivElement>('#router')!.innerHTML = `
            <h1>Post n. ${props.path_params['post_id']}</h1>
        `;
    },
    "/error": (_props: RouteHandlerProps) => {
        throw new RouteError(HttpStatusCode.I_AM_A_TEAPOT, "I am a teapot bozo", null);
    },
}

export function error_route(err: RouteError) {
    document.querySelector<HTMLDivElement>('#router')!.innerHTML = `
        <h1>Error: ${err.httpStatusCode}</h1>
        <hr />
        <p>${err.message}</p>
    `;
}

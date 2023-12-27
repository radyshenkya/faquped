import HttpStatusCode from "./http_status_codes";
import { RouteError, RouteHandlerProps } from "./router";
import { HEADER } from "./components";
import index from "./routes/index";
import signin from "./routes/signin";

export const routes = {
    "/": index,
    "/signin": signin,
    "/error": (_props: RouteHandlerProps) => {
        throw new RouteError(HttpStatusCode.I_AM_A_TEAPOT, "I am a teapot bozo", null);
    },
}

export function error_route(err: RouteError) {
    document.querySelector<HTMLDivElement>('#router')!.innerHTML = `
        ${HEADER}
        <section class="mt-16 container">
            <h1>Error: ${err.httpStatusCode}</h1>
            <hr />
            <p>${err.message}</p>
        </section>
    `;
}

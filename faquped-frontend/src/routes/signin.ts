// import HttpStatusCode from "../http_status_codes";
import { RouteHandlerProps } from "../router";
import { HEADER } from "../components";

export default (_: RouteHandlerProps) => {
    document.querySelector<HTMLDivElement>("#router")!.innerHTML = `
        ${HEADER}
        <section class="container">
            <h1>sign in page</h1>
        </section>
    `;
}

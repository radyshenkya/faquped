import { RouteHandlerProps } from "../router";
import { HEADER } from "../components";

export default (_: RouteHandlerProps) => {
    document.querySelector<HTMLDivElement>("#router")!.innerHTML = `
        ${HEADER}
        <section class="container flex items-center flex-col">
            <h1 class="body-heading mt-16">Sign in</h1>
            <form class="grow font-content">
                <div class="my-1">
                    <label for="password" class="text-slate-600 mx-1 text-sm">
                        Username
                    </label>
                    <input class="text-input block" id="username" placeholder="Username" />
                </div>
                <div class="my-1">
                    <label for="password" class="text-slate-600 mx-1 text-sm">
                        Email
                    </label>
                    <input class="text-input block" id="email" type="email" placeholder="Email" autocomplete="false" />
                </div>
                <div class="my-1">
                    <label for="password" class="text-slate-600 mx-1 text-sm">
                        Password
                    </label>
                    <input class="text-input block" id="password" type="password" placeholder="Password" autocomplete="false" />
                </div>
                <div class="my-4 flex justify-between items-center">
                    <button class="btn-success">Sign in</button>
                    <a class="font-heading text-slate-500 text-lg hover:text-slate-800 transition-all" href="#" data-router-href="/login">Log in page</a>
                </div>
            </form>
        </section>
    `;
}

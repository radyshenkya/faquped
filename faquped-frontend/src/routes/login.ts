import { RouteHandlerProps } from "../router";
import { HEADER } from "../components";
import { showToastByClass } from "../toast-message";

export default (_: RouteHandlerProps) => {
    document.querySelector<HTMLDivElement>("#router")!.innerHTML = `
        ${HEADER}
        <section class="container flex items-center flex-col">
            <h1 class="body-heading mt-16">Log in</h1>
            <div class="grow font-content">
                <div class="my-1">
                    <label for="password" class="text-slate-600 mx-1 text-sm">
                        Username
                    </label>
                    <input class="text-input block" id="username" placeholder="Username" />
                </div>
                <div class="my-1">
                    <label for="password" class="text-slate-600 mx-1 text-sm">
                        Password
                    </label>
                    <input class="text-input block" id="password" type="password" placeholder="Password" autocomplete="false" />
                </div>
                <div class="my-4 flex justify-between items-center">
                    <button class="btn-success" id="login-btn">Log in</button>
                    <a class="font-heading text-slate-500 text-lg hover:text-slate-800 transition-all" href="#" data-router-href="/signin">Sign in page</a>
                </div>
            </div>
        </section>
    `;

    document.querySelector<HTMLButtonElement>("button#login-btn")!.onclick = () => {
        showToastByClass("Че, не работает да?", 4000, "toast-info");
        showToastByClass("Ну я не знаю, выключи компьютер", 6000, "toast-warn");
        showToastByClass("Вообще хз, иди нафиг отсюда", 2000, "toast-error");
        showToastByClass("мяу", 10000, "toast-success");
    };
}

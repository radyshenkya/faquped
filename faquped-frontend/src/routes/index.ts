import { RouteHandlerProps } from "../router";
import { HEADER } from "../components";

export default (_: RouteHandlerProps) => {
    document.querySelector<HTMLDivElement>("#router")!.innerHTML = `
        ${HEADER}
        <section class="mt-24 container lg:flex justify-between">
            <div class="items-start gap-y-8 flex xl:w-3/4 flex-col shrink">
                <h1 class="font-heading font-bold text-5xl">Service for <span class="text-sky-400">FAQ pages</span></h1>
                <p class="font-content text-2xl w-3/4">Create FAQ page for you in under a 5 minutes</p>
                <button data-router-href="/signin" class="btn-primary">Sign in</button>
            </div>
            <div class="flex lg:mt-0 mt-4 flex-col gap-y-4">
                <div class="answer-plate">
                    <h1 class="answer-plate-heading">Answer questions with one link</h1>
                    <p class="answer-plate-body">Fastly create simple FAQ pages for everything you need</p>
                </div>
                <div class="answer-plate">
                    <h1 class="answer-plate-heading">Choose style for you <span class="text-slate-400">coming soon...</span></h1>
                    <p class="answer-plate-body">Various styles are available</p>
                </div>
                <div class="answer-plate">
                    <h1 class="answer-plate-heading">Embedable <span class="text-slate-400">coming soon...</span></h1>
                    <p class="answer-plate-body">Include FAQ pages into your own websites with <span class="font-bold">&ltiframe&gt</span> tag</p>
                </div>
            </div>
        </section>
    `;
}

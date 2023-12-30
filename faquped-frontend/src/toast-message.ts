export enum ToastLevel {
    INFO,
    WARN,
    ERROR,
    SUCCESS,
}

export function showToast(content: string, duration: number) {
    let toastStack = getOrCreateToastStack();

    let newToast = document.createElement("div");
    newToast.innerHTML = content;
    toastStack.appendChild(newToast);
    toastStack.addEventListener("click", (event) => {
        // @ts-ignore
        event.target.remove();
    });

    setTimeout(() => {
        newToast.remove();
    }, duration);
}

export function showToastByClass(text: string, duration: number, className: string) {
    showToast(`
          <div class="${className}">
            ${text}
          </div>
    `, duration);
}

function getOrCreateToastStack(): HTMLDivElement {
    let toastStack = document.querySelector<HTMLDivElement>("div#toast-stack");

    if (toastStack != null) {
        return toastStack;
    }

    let newToastStack = document.createElement("div");
    newToastStack.id = "toast-stack";
    newToastStack.className = "toast-stack";
    document.querySelector<HTMLBodyElement>("body")!.appendChild(newToastStack);

    return newToastStack;
}

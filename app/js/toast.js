async function toast(title, body) {
    document.getElementById("response_title").innerHTML = title;
    document.getElementById("response_body").innerHTML = body;

    const toast = document.getElementById("toast").classList;

    toast.remove("hidden");
    await new Promise((sleep) => setTimeout(sleep, 2024));
    toast.add("hidden");
}

// onload
(function () {
    const body = document.getElementById("body");

    body.innerHTML =
        `
        <div
            id="toast"
            class="fixed right-0 hidden h-20 font-bold text-black bg-white w-80 rounded-xl max-lg:h-16 max-lg:w-60"
        >
            <div class="flex flex-col items-center justify-center h-full">
                <p id="response_title" class="text-lg">Toast Response Title</p>
                <p id="response_body" class="text-sm">Toast Response Body</p>
            </div>
        </div>
        ` + body.innerHTML;
})();

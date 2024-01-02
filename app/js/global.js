(async function () {
    if (location.host == "127.0.0.1:5500") {
        return;
    }

    const token = Cookies.get("token");

    if (token == undefined) {
        if (location.pathname != "/app/html/auth.html") {
            location.href = "auth.html";
        }
        return;
    }

    try {
        const response = await fetch("http://127.0.0.1:42069/api/token", {
            method: "PUT",
        });

        switch (response.status) {
            case 200:
                const response_json = await response.json();
                Cookies.set("token", await response_json.token);
                if (location.pathname != "/app/html/main.html") {
                    location.href = "main.html";
                }
                break;
            default:
                Cookies.remove("token");
                location.href = "auth.html";
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
})();

(function () {
    const body = document.getElementById("body");

    body.innerHTML =
        `
    <div
        id="toast"
        class="fixed right-0 hidden h-20 w-80 rounded-xl bg-white font-bold text-black max-lg:h-16 max-lg:w-60"
    >
        <div class="flex h-full flex-col items-center justify-center">
            <p id="response_title" class="text-lg">Toast Response Title</p>
            <p id="response_body" class="text-sm">Toast Response Body</p>
        </div>
    </div>
        ` + body.innerHTML;
})();

async function toast(title, body) {
    document.getElementById("response_title").innerHTML = title;
    document.getElementById("response_body").innerHTML = body;

    const toast = document.getElementById("toast").classList;

    toast.remove("hidden");
    await new Promise((sleep) => setTimeout(sleep, 2024));
    toast.add("hidden");
}

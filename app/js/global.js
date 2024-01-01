(async function () {
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

async function toast(title, body) {
    document.getElementById("response_title").innerHTML = title;
    document.getElementById("response_body").innerHTML = body;

    const toast = document.getElementById("toast").classList;

    toast.remove("hidden");
    await new Promise((sleep) => setTimeout(sleep, 2024));
    toast.add("hidden");
}

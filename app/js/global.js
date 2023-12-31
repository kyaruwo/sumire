async function toast(title, body) {
    document.getElementById("response_title").innerHTML = title;
    document.getElementById("response_body").innerHTML = body;

    const toast = document.getElementById("toast").classList;

    toast.remove("hidden");
    await new Promise((sleep) => setTimeout(sleep, 2024));
    toast.add("hidden");
}

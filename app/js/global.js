async function toast(response) {
    document.getElementById("response").innerHTML = response;

    const toast = document.getElementById("toast").classList;

    toast.remove("hidden");
    await new Promise((sleep) => setTimeout(sleep, 4269));
    toast.add("hidden");
}

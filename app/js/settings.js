async function logout() {
    try {
        const response = await fetch(
            `http://127.0.0.1:42069/api/users/logout`,
            {
                method: "PUT",
            }
        );

        switch (response.status) {
            case 200:
                Cookies.remove("session_id");
                location.href = "auth";
                break;
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
}

function show_settings() {
    location.hash = "profile";
    document.getElementById("main").innerHTML = /*html*/ `
        <div class="flex flex-col items-center gap-12">
            <div class="flex flex-col gap-1">
                <h id="email" class="text-2xl">username@googlemail.com</h
                ><button
                    onclick="show_change_email()"
                    class="rounded-xl bg-blue-600 px-2 py-1 text-xl"
                >
                    change
                </button>
            </div>
            <div class="flex flex-col gap-1">
                <h id="username" class="text-2xl">username</h>
                <button
                    onclick="show_update_username()"
                    class="rounded-xl bg-blue-600 px-2 py-1 text-xl"
                >
                    update
                </button>
            </div>
            <button
                onclick="show_update_password()"
                class="rounded-xl bg-blue-600 px-8 py-2 text-xl"
            >
                Update Password
            </button>
        </div>
    `;
}

function show_change_email() {
    location.hash = "email";
}
function change_email() {}

function show_new_email() {}
function new_email() {}

function show_update_username() {
    location.hash = "username";
}
function update_username() {}

function show_update_password() {
    location.hash = "password";
}
function update_password() {}

// onload
(async function () {
    const session_id = Cookies.get("session_id");

    if (session_id == undefined) {
        location.href = "auth";
        return;
    }

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/session_id",
            {
                method: "PUT",
            }
        );

        switch (response.status) {
            case 200:
                const response_json = await response.json();
                Cookies.set("session_id", await response_json.session_id);
                break;
            default:
                Cookies.remove("session_id");
                location.href = "auth";
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }

    if (location.hash == "#email") {
        return show_change_email();
    }
    if (location.hash == "#username") {
        return show_update_username();
    }
    if (location.hash == "#password") {
        return show_update_password();
    }
    show_settings();
})();

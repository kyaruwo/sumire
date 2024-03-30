async function logout() {
    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/logout",
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

let profile;

async function show_settings() {
    location.hash = "profile";

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/profile"
        );

        switch (response.status) {
            case 200:
                profile = await response.json();
                break;
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }

    document.getElementById("main").innerHTML = /*html*/ `
        <div class="flex items-center justify-center">
            <div class="flex flex-col items-start gap-12">
                <div class="flex flex-row gap-4">
                    <button
                        onclick="show_change_email()"
                        class="rounded-xl bg-blue-600 px-2 py-1 text-sm"
                    >
                        change
                    </button>
                    <h id="email" class="text-2xl">${profile.email}</h>
                </div>
                <div class="flex flex-row gap-4">
                    <button
                        onclick="show_update_username()"
                        class="rounded-xl bg-blue-600 px-2 py-1 text-sm"
                    >
                        update
                    </button>
                    <h id="username" class="text-2xl">${profile.username}</h>
                </div>
                <div class="flex flex-row gap-4">
                    <button
                        onclick="show_update_password()"
                        class="rounded-xl bg-blue-600 px-2 py-1 text-sm"
                    >
                        update
                    </button>
                    <h class="text-2xl">***sword</h>
                </div>
            </div>
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

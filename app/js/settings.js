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
            <div class="flex flex-col gap-2">
                <span>email</span>
                <div class="flex gap-2 max-md:flex-col lg:flex-row">
                    <h id="email" class="text-2xl">${profile.email}</h>
                    <button
                        onclick="show_change_email_request()"
                        class="rounded-xl bg-blue-600 px-2 py-1 text-sm"
                    >
                        change
                    </button>
                </div>
            </div>
            <div class="flex flex-col gap-2">
                <span>username</span>
                <div class="flex gap-2 max-md:flex-col lg:flex-row">
                    <h id="username" class="text-2xl">${profile.username}</h>
                    <button
                        onclick="show_update_username()"
                        class="rounded-xl bg-blue-600 px-2 py-1 text-sm"
                    >
                        update
                    </button>
                </div>
            </div>
            <div class="flex flex-col gap-2">
                <span>password</span>
                <div class="flex gap-2 max-md:flex-col lg:flex-row">
                    <h class="text-2xl">********</h>
                    <button
                        onclick="show_update_password()"
                        class="rounded-xl bg-blue-600 px-2 py-1 text-sm"
                    >
                        update
                    </button>
                </div>
            </div>
        </div>
    </div>
    `;
}

function show_change_email_request() {
    document.getElementById("main").innerHTML = /*html*/ `
    <div class="flex flex-col items-center gap-2">
    <form
            onsubmit="change_email_request();return false"
            class="flex flex-col gap-4 text-center text-xl"
        >
            <h class="mb-2 px-8 text-4xl font-bold">Change Email</h>
            <input
                id="email"
                type="text"
                placeholder="email"
                minlength="16"
                maxlength="45"
                required
                class="rounded-xl bg-neutral-950 p-4 lowercase"
                disabled
                value="${profile.email}"
            />
            <button class="rounded-xl bg-green-600 px-6 py-4 text-xl font-bold">
                send code
            </button>

            <hr class="my-4" />
                
            <button
                onclick="show_settings();return false"
                class="rounded-xl bg-red-600 py-2 font-bold"
            >
                Cancel
            </button>
        </form>
    </div>
    `;
}
function change_email_request() {}

function show_new_email() {
    document.getElementById("main").innerHTML = /*html*/ `
    <div class="flex flex-col items-center gap-2">
    <form
            onsubmit="new_email();return false"
            class="flex flex-col gap-4 text-center text-xl"
        >
            <h class="mb-2 px-12 text-4xl font-bold">New Email</h>
            <input
                id="email"
                type="text"
                placeholder="email"
                minlength="16"
                maxlength="45"
                required
                class="rounded-xl bg-neutral-950 p-4 lowercase"
                disabled
                value="${profile.email}"
            />
            <input
                id="code"
                type="number"
                placeholder="code"
                min="10000000"
                max="99999999"
                required
                class="rounded-xl p-4 text-black [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
            />

            <hr class="my-4" />

            <input
                id="new_email"
                type="text"
                placeholder="new_email"
                minlength="16"
                maxlength="45"
                required
                class="rounded-xl p-4 lowercase text-black"
            />
            <button class="rounded-xl bg-blue-600 px-6 py-4 text-xl font-bold">
                Change
            </button>

            <hr class="my-4" />

            <button
                onclick="show_settings();return false"
                class="rounded-xl bg-red-600 py-2 font-bold"
            >
                Cancel
            </button>
        </form>
    </div>
    `;
}
function new_email() {}

function show_update_username() {
    document.getElementById("main").innerHTML = /*html*/ `
        <div class="flex flex-col items-center">
            <form
                onsubmit="update_username();return false"
                class="flex flex-col gap-4 text-center text-xl"
            >
                <h class="mb-2 text-4xl font-bold lg:px-10">Update Username</h>
                <input
                    id="username"
                    type="text"
                    placeholder="username"
                    minlength="4"
                    maxlength="20"
                    required
                    class="my-4 rounded-xl p-4 lowercase text-black"
                    onkeydown="return /[a-z]/i.test(event.key)"
                    onpaste="return false;"
                    ondrop="return false;"
                    value="${profile.username}"
                />
                <button class="rounded-xl bg-blue-600 py-2 font-bold">
                    Update
                </button>

                <hr class="my-4" />
                
                <button
                    onclick="show_settings();return false"
                    class="rounded-xl bg-red-600 py-2 font-bold"
                >
                    Cancel
                </button>
            </form>
        </div>
    `;
}
async function update_username() {
    const data = {
        username: document.getElementById("username").value.toLowerCase(),
    };

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/username",
            {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
            }
        );

        switch (response.status) {
            case 200:
                toast("Success", "Username Updated");
                show_settings();
                break;
            case 409:
                toast("Conflict", await response.text());
                break;
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
}

function show_update_password() {
    document.getElementById("main").innerHTML = /*html*/ `
    <div class="flex flex-col items-center">
        <form
            onsubmit="update_password();return false"
            class="flex flex-col gap-4 text-center text-xl"
        >
            <h class="mb-2 text-4xl font-bold lg:px-10">Update Password</h>
            <input
                id="password"
                type="password"
                placeholder="password"
                minlength="11"
                maxlength="69"
                required
                class="my-2 rounded-xl p-4 text-black"
            />
            <input
                id="new_password"
                type="password"
                placeholder="new_password"
                minlength="11"
                maxlength="69"
                required
                class="rounded-xl p-4 text-black"
            />
            <input
                id="verify_new_password"
                type="password"
                placeholder="verify_new_password"
                minlength="11"
                maxlength="69"
                required
                class="rounded-xl p-4 text-black"
            />
            <button class="mt-2 rounded-xl bg-blue-600 py-2 font-bold">
                Update
            </button>

            <hr class="my-4" />

            <button
                onclick="show_settings();return false"
                class="rounded-xl bg-red-600 py-2 font-bold"
            >
                Cancel
            </button>
        </form>
    </div>
`;
}
async function update_password() {
    const new_password = document.getElementById("new_password").value;
    if (new_password != document.getElementById("verify_new_password").value) {
        return toast("Mismatch", "Password");
    }

    const data = {
        old_password: document.getElementById("password").value,
        new_password: new_password,
    };

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/password",
            {
                method: "PUT",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
            }
        );

        switch (response.status) {
            case 200:
                toast("Success", "Password Updated");
                show_settings();
                break;
            case 404:
                toast("Not Found", "Incorrect Password");
                break;
            case 401:
                return logout();
            default:
                toast("Error", "An error occurred");
                break;
        }
    } catch (e) {
        console.log(e);
        toast("Error", "An error occurred");
    }
}

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

    show_settings();
})();

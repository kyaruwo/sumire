function show_register() {
    location.hash = "register";
    document.getElementById("main").innerHTML = /*html*/ `
        <form
            onsubmit="register();return false"
            class="flex flex-col gap-2 text-xl"
        >
            <h class="mb-2 text-4xl font-bold lg:px-12">Create Account</h>
            <input
                id="email"
                type="text"
                placeholder="email"
                minlength="16"
                maxlength="45"
                required
                class="rounded-xl p-4 lowercase text-black"
            />
            <input
                id="username"
                type="text"
                placeholder="username"
                minlength="4"
                maxlength="20"
                required
                class="rounded-xl p-4 lowercase text-black"
            />
            <input
                id="password"
                type="password"
                placeholder="password"
                minlength="11"
                maxlength="69"
                required
                class="rounded-xl p-4 text-black"
            />
            <input
                id="verify_password"
                type="password"
                placeholder="verify password"
                minlength="11"
                maxlength="69"
                required
                class="rounded-xl p-4 text-black"
            />
            <button class="rounded-xl bg-green-600 py-4 font-bold">
                Register
            </button>
        </form>

        <hr class="my-4" />

        <button
            onclick="show_login()"
            class="rounded-xl bg-blue-600 px-24 py-4 text-xl font-bold"
        >
            Login
        </button>
    `;
}

async function register() {
    let password = document.getElementById("password").value;
    if (password != document.getElementById("verify_password").value) {
        return toast("Mismatch", "Password");
    }

    const data = {
        email: document.getElementById("email").value.toLowerCase(),
        username: document.getElementById("username").value.toLowerCase(),
        password: password,
    };

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/register",
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
            }
        );

        switch (response.status) {
            case 201:
                toast("Success", "Account created");
                show_verify();
                document.getElementById("email").value = data.email;
                break;
            case 409:
                toast("Conflict", await response.text());
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

function show_login() {
    location.hash = "login";
    document.getElementById("main").innerHTML = /*html*/ `
        <form
            onsubmit="login();return false"
            class="flex flex-col gap-4 text-xl"
        >
            <h class="mb-2 text-4xl font-bold lg:px-10">Login Account</h>
            <input
                id="username"
                type="text"
                placeholder="username"
                minlength="4"
                maxlength="20"
                required
                class="rounded-xl p-4 lowercase text-black"
            />
            <input
                id="password"
                type="password"
                placeholder="password"
                minlength="11"
                maxlength="69"
                required
                class="rounded-xl p-4 text-black"
            />
            <button class="rounded-xl bg-blue-600 py-4 font-bold">Login</button>
        </form>

        <button onclick="show_forgot_password()" class="mt-4 text-sm">
            Forgot Password?
        </button>

        <hr class="my-4" />

        <button
            onclick="show_register()"
            class="rounded-xl bg-green-600 px-20 py-4 text-xl font-bold"
        >
            Register
        </button>
    `;
}

function show_forgot_password() {
    location.hash = "forgot_password";
    document.getElementById("main").innerHTML = /*html*/ `
        <form
            onsubmit="forgot_password();return false"
            class="flex flex-col gap-4 text-xl"
        >
            <h class="mb-2 text-4xl font-bold lg:px-10">Forgot Password</h>
            <input
                id="email"
                type="text"
                placeholder="email"
                minlength="16"
                maxlength="45"
                required
                class="rounded-xl p-4 lowercase text-black"
            />
            <button class="rounded-xl bg-green-600 px-6 py-4 text-xl font-bold">
                send code
            </button>
        </form>

        <hr class="my-4" />

        <button
            onclick="show_login()"
            class="rounded-xl bg-blue-600 px-12 py-4 text-xl font-bold"
        >
            Login
        </button>
    `;
}

async function forgot_password() {
    const data = {
        email: document.getElementById("email").value.toLowerCase(),
    };
    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/forgot_password",
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
            }
        );

        switch (response.status) {
            case 200:
                toast("Success", "code sent");
                show_new_password();
                document.getElementById("email").value = data.email;
                break;
            case 404:
                toast("Not Found", "Incorrect email");
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

function show_new_password() {
    document.getElementById("main").innerHTML = /*html*/ `
        <form
            onsubmit="new_password();return false"
            class="flex flex-col gap-4 text-xl"
        >
            <h class="mb-2 text-4xl font-bold lg:px-10">New Password</h>
            <input
                id="email"
                type="text"
                placeholder="email"
                minlength="16"
                maxlength="45"
                required
                class="rounded-xl bg-neutral-950 p-4 lowercase"
                disabled
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
            <input
                id="password"
                type="password"
                placeholder="password"
                minlength="11"
                maxlength="69"
                required
                class="rounded-xl p-4 text-black"
            />
            <input
                id="verify_password"
                type="password"
                placeholder="verify_password"
                minlength="11"
                maxlength="69"
                required
                class="rounded-xl p-4 text-black"
            />
            <button class="rounded-xl bg-blue-600 px-6 py-4 text-xl font-bold">
                Save
            </button>
        </form>
    `;
}

async function new_password() {
    let password = document.getElementById("password").value;
    if (password != document.getElementById("verify_password").value) {
        return toast("Mismatch", "Password");
    }

    const data = {
        email: document.getElementById("email").value.toLowerCase(),
        new_password: password,
        code: parseInt(document.getElementById("code").value),
    };

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/new_password",
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
                await toast("Success", "Redirecting");
                location.hash = "login";
                show_login();
                break;
            case 404:
                toast("Not Found", "Incorrect code");
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

function show_verify() {
    document.getElementById("main").innerHTML = /*html*/ `
        <form
            onsubmit="verify();return false"
            class="flex flex-col gap-4 text-xl"
        >
            <h class="mb-2 text-4xl font-bold lg:px-10">Verify Account</h>
            <input
                id="email"
                type="text"
                placeholder="email"
                minlength="16"
                maxlength="45"
                required
                class="rounded-xl bg-neutral-950 p-4 lowercase"
                disabled
            />
            <button
                onclick="code_request();return false"
                class="rounded-xl bg-green-600 px-6 py-4 text-xl font-bold"
            >
                send code
            </button>
            <input
                id="code"
                type="number"
                placeholder="code"
                min="10000000"
                max="99999999"
                required
                class="rounded-xl p-4 text-black [appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
            />
            <button class="rounded-xl bg-blue-600 py-4 font-bold">
                Verify
            </button>
        </form>
    `;
}

async function code_request() {
    const data = {
        email: document.getElementById("email").value.toLowerCase(),
    };
    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/code_request",
            {
                method: "POST",
                headers: {
                    "Content-Type": "application/json",
                },
                body: JSON.stringify(data),
            }
        );

        switch (response.status) {
            case 200:
                toast("Success", "code sent");
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

async function verify() {
    const data = {
        email: document.getElementById("email").value.toLowerCase(),
        code: parseInt(document.getElementById("code").value),
    };

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/verify_email",
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
                await toast("Success", "Redirecting");
                location.hash = "login";
                show_login();
                break;
            case 404:
                toast("Not Found", "Incorrect code");
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

async function login() {
    const data = {
        username: document.getElementById("username").value.toLowerCase(),
        password: document.getElementById("password").value,
    };

    try {
        const response = await fetch("http://127.0.0.1:42069/api/users/login", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(data),
        });

        switch (response.status) {
            case 200:
                const response_json = await response.json();
                Cookies.set("session_id", await response_json.session_id);
                await toast("Success", "Redirecting");
                location.href = "notes";
                break;
            case 401:
                toast("Not Verified", "Redirecting");
                show_verify();
                document.getElementById("email").value = await response.text();
                break;
            case 404:
                toast("Not Found", "Incorrect name or password");
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

// onload
(async function () {
    const session_id = Cookies.get("session_id");
    if (session_id != undefined) {
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
                    location.href = "notes";
                    break;
                default:
                    Cookies.remove("session_id");
                    break;
            }
        } catch (e) {
            console.log(e);
            toast("Error", "An error occurred");
        }
    }

    if (location.hash == "#register") {
        return show_register();
    }
    if (location.hash == "#forgot_password") {
        return show_forgot_password();
    }
    show_login();
})();

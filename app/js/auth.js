function show_register() {
    location.hash = "register";
    document.getElementById("main").innerHTML = `
    <form
        onsubmit="register();return false"
        class="flex flex-col gap-4 text-xl"
    >
        <h class="mb-2 text-4xl font-bold lg:px-8"
            >Create Account</h
        >
        <input
            id="name"
            type="text"
            placeholder="name"
            minlength="4"
            maxlength="20"
            required
            class="rounded-xl p-4 lowercase text-black"
        />
        <input
            id="password"
            type="password"
            placeholder="password"
            minlength="8"
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
    const data = {
        name: document.getElementById("name").value.toLowerCase(),
        password: document.getElementById("password").value,
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
                show_login();
                break;
            case 409:
                toast("Conflict", "Account already exists");
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
    document.getElementById("main").innerHTML = `
    <form
        onsubmit="login();return false"
        class="flex flex-col gap-4 text-xl"
    >
        <h class="mb-2 text-4xl font-bold lg:px-10"
            >Login Account</h
        >
        <input
            id="name"
            type="text"
            placeholder="name"
            minlength="4"
            maxlength="20"
            required
            class="rounded-xl p-4 lowercase text-black"
        />
        <input
            id="password"
            type="password"
            placeholder="password"
            minlength="8"
            maxlength="69"
            required
            class="rounded-xl p-4 text-black"
        />
        <button class="rounded-xl bg-blue-600 py-4 font-bold">
            Login
        </button>
    </form>

    <hr class="my-4" />

    <button
        onclick="show_register()"
        class="rounded-xl bg-green-600 px-20 py-4 text-xl font-bold"
    >
        Register
    </button>
    `;
}

async function login() {
    const data = {
        name: document.getElementById("name").value.toLowerCase(),
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
                Cookies.set("token", await response_json.token);
                await toast("Success", "Redirecting");
                location.href = "main.html";
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
    const token = Cookies.get("token");
    if (token != undefined) {
        try {
            const response = await fetch("http://127.0.0.1:42069/api/token", {
                method: "PUT",
            });

            switch (response.status) {
                case 200:
                    const response_json = await response.json();
                    Cookies.set("token", await response_json.token);
                    location.href = "main.html";
                    break;
                default:
                    Cookies.remove("token");
                    break;
            }
        } catch (e) {
            console.log(e);
            toast("Error", "An error occurred");
        }
    }

    if (location.hash == "#register") {
        show_register();
    } else {
        show_login();
    }
})();

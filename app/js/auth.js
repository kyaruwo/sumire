if (location.hash == "#register") {
    show_register();
} else {
    show_login();
}

function show_register() {
    location.hash = "register";
    document.getElementById("main").innerHTML = `
    <form onsubmit="register();return false" class="flex flex-col gap-4 text-xl">
        <h class="mb-2 text-4xl font-bold lg:px-8">Create Account</h>
        <input
            id="name"
            type="text"
            placeholder="name"
            required
            class="rounded-xl p-4 text-black"
        />
        <input
            id="password"
            type="password"
            placeholder="password"
            required
            class="rounded-xl p-4 text-black"
        />
        <button class="rounded-xl bg-green-600 py-4 font-bold">
            Register
        </button>
    </form>

    <hr class="my-4"></hr>

    <button onclick="show_login()" class="rounded-xl bg-blue-600 px-24 py-4 text-xl font-bold">Login</button>
    `;
}

function show_login() {
    location.hash = "login";
    document.getElementById("main").innerHTML = `
    <form onsubmit="login();return false" class="flex flex-col gap-4 text-xl">
        <h class="mb-2 text-4xl font-bold lg:px-10">Login Account</h>
        <input
            id="name"
            type="text"
            placeholder="name"
            required
            class="rounded-xl p-4 text-black"
        />
        <input
            id="password"
            type="password"
            placeholder="password"
            required
            class="rounded-xl p-4 text-black"
        />
        <button class="rounded-xl bg-blue-600 py-4 font-bold">
            Login
        </button>
    </form>

    <hr class="my-4"></hr>

    <button onclick="show_register()" class="rounded-xl bg-green-600 px-20 py-4 text-xl font-bold">Register</button>
    `;
}

function register() {
    console.log("register");
}

function login() {
    console.log("login");
}

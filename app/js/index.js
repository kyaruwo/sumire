if (location.hash == "#about") {
    show_about();
} else {
    show_sumire();
}

function show_sumire() {
    location.hash = "sumire";
    const main = document.getElementById("main");
    main.innerHTML = `
    <h class="text-8xl max-lg:text-7xl max-md:text-6xl max-sm:text-5xl">
        Write down Notes
    </h>
    `;

    if (!Cookies.get("token")) {
        main.innerHTML += `    
        <a
            class="px-24 py-3 mt-12 mb-2 text-2xl bg-green-600 rounded-full max-md:px-20 max-md:text-xl"
            href="/app/html/auth.html#register"
        >
            Register
        </a>
        <p class="max-md:text-sm">
            Already have an account?
            <a class="text-blue-400" href="/app/html/auth.html#login">Login</a>
        </p>
        `;
    } else {
        main.innerHTML += `    
        <a
            class="px-24 py-3 mt-12 mb-2 text-2xl bg-green-600 rounded-full max-md:px-20 max-md:text-xl"
            href="/app/html/main.html"
        >
            Write
        </a>
        `;
    }
}

function show_about() {
    location.hash = "about";
    document.getElementById("main").innerHTML = `
    <a href="https://kyaruwo.github.io/" target="_blank">
        <img
            src="/app/assets/tako.gif"
            class="-mt-12 max-md:size-40 max-lg:size-52 lg:size-60"
    /></a>
    <h class="mt-2 text-4xl font-bold">kyaruwo</h>
    <p class="text-xl">Developer</p>
    `;
}

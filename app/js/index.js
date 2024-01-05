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
            href="/app/html/auth.html#register"
            class="mb-2 mt-12 rounded-full bg-green-600 px-24 py-3 text-2xl max-md:px-20 max-md:text-xl"
        >
            Register
        </a>
        <p class="max-md:text-sm">
            Already have an account?
            <a href="/app/html/auth.html#login" class="text-blue-400">Login</a>
        </p>
        `;
    } else {
        main.innerHTML += `    
        <a
            href="/app/html/main.html"
            class="mb-2 mt-12 rounded-full bg-green-600 px-24 py-3 text-2xl max-md:px-20 max-md:text-xl"
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
            class="max-md:size-40 max-lg:size-52 lg:size-60 -mt-12"
    /></a>
    <h class="mt-2 text-4xl font-bold">kyaruwo</h>
    <p class="text-xl">Developer</p>
    `;
}

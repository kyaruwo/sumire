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
    if (location.hash == "#verify") {
        show_verify();
        document.getElementById("email").value = new URL(
            document.location
        ).searchParams.get("email");
        return;
    }
    show_login();
})();

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

async function change_email_request() {
    const data = {
        email: document.getElementById("email").value,
    };

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/change_email_request",
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
                show_change_email();
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

async function change_email() {
    const data = {
        old_email: document.getElementById("email").value,
        code: parseInt(document.getElementById("code").value),
        new_email: document.getElementById("new_email").value,
    };

    try {
        const response = await fetch(
            "http://127.0.0.1:42069/api/users/change_email",
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
                toast("Success", "Email Changed");
                Cookies.remove("session_id");
                location.href = `auth?email=${data.new_email}#verify`;
                break;
            case 404:
                toast("Not Found", "Incorrect code");
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
